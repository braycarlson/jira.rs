use core::time::Duration;
use std::io::{Read as _, Seek, Write};
use std::thread;

use ureq::config::Config;
#[cfg(test)]
use ureq::unversioned::resolver::Resolver;
#[cfg(test)]
use ureq::unversioned::transport::Connector;
use ureq::{AsSendBody, http};

use crate::client::{STATUS_MAX, STATUS_MIN, status_is_success};
use crate::error::Error;
use crate::limits;
use crate::method::Method;
use crate::request::Request;
use crate::response::Response;
use crate::size;
use crate::text;
use crate::transport::Transport;
use crate::wait::Wait;

const CONNECTION_IDLE_COUNT_MAX: u64 = 16;
const COPY_BUFFER_BYTES: u64 = 64 * 1024;
const HEADER_BYTES_MAX: u64 = 2_048;
const DOWNLOAD_TIMEOUT_SECONDS: u64 = 3_600;
const IDLE_TIMEOUT_SECONDS: u64 = 60;
const REDIRECT_COUNT_MAX: u32 = 8;
const REQUEST_TIMEOUT_SECONDS: u64 = 60;
const SCHEME: &str = "https://";
const UPLOAD_BYTES_PER_SECOND_MIN: u64 = 64 * 1024;
const _: () = assert!(CONNECTION_IDLE_COUNT_MAX >= 1, "a client keeps at least one connection");
const _: () = assert!(REQUEST_TIMEOUT_SECONDS >= 1, "a request gets at least a second");
const _: () = assert!(IDLE_TIMEOUT_SECONDS >= 1, "a quiet link gets at least a second");

const _: () =
    assert!(IDLE_TIMEOUT_SECONDS <= DOWNLOAD_TIMEOUT_SECONDS, "a stall ends before the download");

const _: () =
    assert!(DOWNLOAD_TIMEOUT_SECONDS >= REQUEST_TIMEOUT_SECONDS, "a download outlasts a request");

const _: () = assert!(STATUS_MIN < STATUS_MAX, "the status range is ordered");
const _: () = assert!(COPY_BUFFER_BYTES >= 1, "a copy moves at least one byte");
const _: () = assert!(UPLOAD_BYTES_PER_SECOND_MIN >= 1, "an upload is given a floor rate");

#[derive(Debug)]
pub struct Agent {
    ureq: ureq::Agent,
}

#[derive(Clone, Copy, Debug)]
struct Hop<'url> {
    is_signed: bool,
    url: &'url str,
}

#[derive(Clone, Copy, Debug)]
struct Reference<'text> {
    base: &'text str,
    target: &'text str,
}

impl Agent {
    pub(crate) fn call(
        &self,
        request: &Request,
        timeout: Duration,
    ) -> Result<http::Response<ureq::Body>, Error> {
        assert!(!timeout.is_zero(), "a call is given time to finish");
        assert!(request.url().starts_with(SCHEME), "every request is encrypted");

        let first = Hop { is_signed: true, url: request.url() };

        let answer = if request.body().is_empty() {
            self.run(self.outbound(request, first, ())?, timeout)
        } else {
            self.run(self.outbound(request, first, request.body())?, timeout)
        };

        let response = answer.map_err(|cause| failure_read(&cause))?;

        if request.method() != Method::Get {
            return status_checked(response);
        }

        status_checked(self.redirects_followed(request, response, timeout)?)
    }

    fn config(idle: Duration) -> Config {
        assert!(!idle.is_zero(), "a quiet link is given time to speak");

        let config = ureq::Agent::config_builder()
            .http_status_as_error(false)
            .https_only(true)
            .max_redirects(0)
            .timeout_per_read(Some(idle))
            .max_idle_connections(size::offset(CONNECTION_IDLE_COUNT_MAX))
            .max_idle_connections_per_host(size::offset(CONNECTION_IDLE_COUNT_MAX))
            .build();

        assert!(config.https_only(), "a request never leaves TLS");
        assert_eq!(config.max_redirects(), 0, "a redirect is followed here and never inside ureq");

        config
    }

    #[must_use]
    pub fn new() -> Self {
        let idle = Duration::from_secs(IDLE_TIMEOUT_SECONDS);
        let agent = Self { ureq: Self::config(idle).new_agent() };

        assert!(!agent.ureq.config().http_status_as_error(), "a refusal is read, not raised");
        assert!(agent.ureq.config().max_idle_connections() >= 1, "a client keeps a connection");

        agent
    }

    fn outbound<Payload>(
        &self,
        request: &Request,
        hop: Hop<'_>,
        body: Payload,
    ) -> Result<http::Request<Payload>, Error> {
        assert!(!hop.url.is_empty(), "a request names where it goes");
        assert!(self.ureq.config().max_idle_connections() >= 1, "an agent keeps a connection");

        let mut builder = http::Request::builder()
            .method(request.method().text())
            .uri(hop.url)
            .header("Accept", request.accept());

        if hop.is_signed {
            builder = builder.header("Authorization", request.authorization());
        }

        if let Some(content_type) = request.content_type() {
            builder = builder.header("Content-Type", content_type);
        }

        for (name, value) in request.headers() {
            builder = builder.header(*name, value.as_str());
        }

        builder.body(body).map_err(|cause| Error::Input(format!("a bad request: {cause}")))
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn over<Over, Resolving>(connector: Over, resolver: Resolving) -> Self
    where
        Over: Connector,
        Resolving: Resolver,
    {
        Self::over_idle(connector, resolver, Duration::from_secs(IDLE_TIMEOUT_SECONDS))
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn over_idle<Over, Resolving>(
        connector: Over,
        resolver: Resolving,
        idle: Duration,
    ) -> Self
    where
        Over: Connector,
        Resolving: Resolver,
    {
        assert!(!idle.is_zero(), "a quiet link is given time to speak");

        let agent = Self { ureq: ureq::Agent::with_parts(Self::config(idle), connector, resolver) };

        assert!(agent.ureq.config().https_only(), "a test agent keeps the real configuration");

        agent
    }

    fn redirects_followed(
        &self,
        request: &Request,
        first: http::Response<ureq::Body>,
        timeout: Duration,
    ) -> Result<http::Response<ureq::Body>, Error> {
        assert_eq!(request.method(), Method::Get, "only a GET follows a redirect");
        assert!(request.body().is_empty(), "a GET carries no body to send again");
        assert!(first.status().as_u16() >= STATUS_MIN, "an answer carries a real status");
        assert!(!timeout.is_zero(), "a call is given time to finish");

        let mut is_signed = true;
        let mut response = first;
        let mut url = request.url().to_owned();

        for _hop_index in 0..REDIRECT_COUNT_MAX {
            if !status_is_redirect(response.status().as_u16()) {
                return Ok(response);
            }

            url = location_read(&response, &url)?;
            is_signed = is_signed && origin_of(&url) == origin_of(request.url());

            let hop = Hop { is_signed, url: &url };
            let answer = self.run(self.outbound(request, hop, ())?, timeout);

            response = answer.map_err(|cause| failure_read(&cause))?;
        }

        if status_is_redirect(response.status().as_u16()) {
            return Err(Error::Reply(format!("too many redirects from {}", request.url())));
        }

        Ok(response)
    }

    fn run<Payload>(
        &self,
        outbound: http::Request<Payload>,
        timeout: Duration,
    ) -> Result<http::Response<ureq::Body>, ureq::Error>
    where
        Payload: AsSendBody,
    {
        assert!(!timeout.is_zero(), "a call is given time to finish");
        assert_eq!(self.ureq.config().max_redirects(), 0, "ureq never follows a redirect");

        let configured =
            self.ureq.configure_request(outbound).timeout_global(Some(timeout)).build();

        self.ureq.run(configured)
    }
}

impl Default for Agent {
    fn default() -> Self {
        Self::new()
    }
}

impl Transport for Agent {
    fn fetch(&self, request: &Request, bytes_max: u64) -> Result<Response, Error> {
        assert!(bytes_max >= 1, "a body may hold at least one byte");
        assert!(bytes_max <= limits::BODY_BYTES_MAX, "a body fits the largest page");

        let mut answer = self.call(request, timeout_of(request))?;

        if !status_is_success(answer.status().as_u16(), request) {
            let refusal = refusal_read(&mut answer);

            return Ok(response_read(&answer, refusal, 0));
        }

        let mut body = Vec::new();
        let limit = bytes_max.strict_add(1);
        let failure = answer.body_mut().as_reader().take(limit).read_to_end(&mut body).err();

        if size::of_slice(&body) > bytes_max {
            if request.is_read() {
                return Err(Error::Reply(format!("a reply ran past the {bytes_max} byte limit")));
            }

            body.truncate(size::offset(bytes_max));
        }

        let status = answer.status().as_u16();

        if let Some(cause) = failure.filter(|_cause| body_loss_matters(status, request)) {
            return Err(Error::Transport(format!("body read failed: {cause}")));
        }

        assert!(size::of_slice(&body) <= bytes_max, "a body never passes its limit");

        Ok(response_read(&answer, body, 0))
    }

    fn stream<Sink>(&self, request: &Request, sink: &mut Sink) -> Result<Response, Error>
    where
        Sink: Seek + Write,
    {
        assert!(request.url().starts_with(SCHEME), "every request is encrypted");

        let mut answer = self.call(request, Duration::from_secs(DOWNLOAD_TIMEOUT_SECONDS))?;

        let status = answer.status().as_u16();

        if status_holds_no_body(status) {
            return Ok(response_read(&answer, Vec::new(), 0));
        }

        if !status_holds_file(status, request) {
            let refusal = refusal_read(&mut answer);

            let is_nothing = refusal.is_empty() && status_is_success(status, request);

            if is_nothing {
                let url = request.url();

                return Err(Error::Reply(format!("{url} answered {status} with no file")));
            }

            return Ok(response_read(&answer, refusal, 0));
        }

        let streamed_count = body_copy(&mut answer, sink)?;

        assert!(
            streamed_count <= limits::ATTACHMENT_BYTES_MAX.strict_add(1),
            "a download stops one byte past its guard",
        );

        Ok(response_read(&answer, Vec::new(), streamed_count))
    }

    fn wait(&self, wait: &Wait<'_>) -> Result<(), Error> {
        assert!(!wait.reason.is_empty(), "a wait always says why");
        assert!(self.ureq.config().max_idle_connections() >= 1, "an agent keeps a connection");

        thread::sleep(wait.delay);

        Ok(())
    }
}

fn body_copy<Sink>(answer: &mut http::Response<ureq::Body>, sink: &mut Sink) -> Result<u64, Error>
where
    Sink: Write,
{
    let guard = limits::ATTACHMENT_BYTES_MAX.strict_add(1);
    let mut reader = answer.body_mut().as_reader().take(guard);
    let mut buffer = vec![0_u8; size::offset(COPY_BUFFER_BYTES)];
    let mut copied: u64 = 0;

    assert!(!buffer.is_empty(), "a copy has room for a byte");

    for _read_index in 0..guard {
        let read_count = reader
            .read(&mut buffer)
            .map_err(|cause| Error::Transport(format!("body read failed: {cause}")))?;

        if read_count == 0 {
            break;
        }

        let chunk = buffer.get(..read_count).unwrap_or_default();

        sink.write_all(chunk)
            .map_err(|cause| Error::Input(format!("cannot write the download: {cause}")))?;

        copied = copied.strict_add(size::of_offset(read_count));
    }

    assert!(copied <= guard, "a copy stops at its guard");

    Ok(copied)
}

fn body_loss_matters(status: u16, request: &Request) -> bool {
    assert!(status >= STATUS_MIN, "an answer carries a real status");
    assert!(!request.url().is_empty(), "a body answers a request");

    if !request.is_read() {
        return false;
    }

    !status_holds_no_body(status)
}

const fn byte_is_header_name(byte: u8) -> bool {
    if byte.is_ascii_lowercase() {
        return true;
    }

    assert!(!byte.is_ascii_lowercase(), "a lowercase letter already passed");

    byte == b'-'
}

fn failure_read(cause: &ureq::Error) -> Error {
    let message = cause.to_string();

    assert!(!message.is_empty(), "a failure says what happened");

    if matches!(
        cause,
        ureq::Error::BadUri(_)
            | ureq::Error::LargeResponseHeader(..)
            | ureq::Error::RequireHttpsOnly(_),
    ) {
        return Error::Reply(message);
    }

    Error::Transport(message)
}

fn header_flag(response: &http::Response<ureq::Body>, name: &str) -> bool {
    assert!(name.bytes().all(byte_is_header_name), "a header name is lowercase");

    response
        .headers()
        .get(name)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.eq_ignore_ascii_case("true"))
}

fn header_number(response: &http::Response<ureq::Body>, name: &str) -> Option<u64> {
    assert!(name.bytes().all(byte_is_header_name), "a header name is lowercase");

    response
        .headers()
        .get(name)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok())
}

fn header_text(response: &http::Response<ureq::Body>, name: &str) -> Option<String> {
    assert!(name.bytes().all(byte_is_header_name), "a header name is lowercase");

    let value = response.headers().get(name)?.to_str().ok()?;

    if value.is_empty() {
        return None;
    }

    Some(text::prefix(value, HEADER_BYTES_MAX).to_owned())
}

fn header_whole(response: &http::Response<ureq::Body>, name: &str) -> Option<String> {
    assert!(name.bytes().all(byte_is_header_name), "a header name is lowercase");

    let value = String::from_utf8_lossy(response.headers().get(name)?.as_bytes()).into_owned();

    if value.is_empty() {
        return None;
    }

    if size::of_text(&value) > u64::from(limits::URL_BYTES_MAX) {
        return None;
    }

    assert!(!value.is_empty(), "a kept header says something");

    Some(value)
}

fn location_read(answer: &http::Response<ureq::Body>, base: &str) -> Result<String, Error> {
    assert!(!base.is_empty(), "a redirect leaves an address");

    let Some(location) = header_whole(answer, "location") else {
        return Err(Error::Reply(format!("a redirect from {base} names nowhere to go")));
    };

    let target = location.split('#').next().unwrap_or_default();
    let resolved = reference_resolved(Reference { base, target });

    let Ok(uri) = resolved.parse::<http::Uri>() else {
        return Err(Error::Reply(format!(
            "a redirect from {base} names a bad address: {location}"
        )));
    };

    Ok(uri.to_string())
}

fn origin_of(url: &str) -> &str {
    assert!(!url.is_empty(), "an address is named");

    let scheme = url.get(..SCHEME.len()).filter(|head| head.eq_ignore_ascii_case(SCHEME));
    let rest = url.get(scheme.map_or(0, str::len)..).unwrap_or_default();
    let tail = rest.find(['/', '?']).and_then(|at| rest.get(at..)).unwrap_or_default();

    url.strip_suffix(tail).unwrap_or(url)
}

fn reference_is_absolute(target: &str) -> bool {
    assert!(!target.contains('#'), "a fragment never reaches a request");

    let Some((scheme, _rest)) = target.split_once(':') else {
        return false;
    };

    let mut bytes = scheme.bytes();

    if !bytes.next().is_some_and(|byte| byte.is_ascii_alphabetic()) {
        return false;
    }

    bytes.all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'+' | b'-' | b'.'))
}

fn reference_merged(reference: Reference<'_>) -> String {
    assert!(!reference.base.is_empty(), "a redirect leaves an address");
    assert!(!reference.target.is_empty(), "an empty reference names the base itself");

    let Reference { base, target } = reference;

    if reference_is_absolute(target) {
        return target.to_owned();
    }

    if target.starts_with("//") {
        return format!("https:{target}");
    }

    let origin = origin_of(base);

    if target.starts_with('/') {
        return format!("{origin}{target}");
    }

    let tail = base.get(origin.len()..).unwrap_or_default();
    let path = tail.split('?').next().unwrap_or_default();

    if target.starts_with('?') {
        return format!("{origin}{path}{target}");
    }

    let directory = path.rsplit_once('/').map_or("", |(head, _leaf)| head);

    format!("{origin}{directory}/{target}")
}

fn reference_resolved(reference: Reference<'_>) -> String {
    assert!(!reference.base.is_empty(), "a redirect leaves an address");
    assert!(!reference.target.contains('#'), "a fragment never reaches a request");

    if reference.target.is_empty() {
        return reference.base.to_owned();
    }

    let merged = reference_merged(reference);
    let origin = origin_of(&merged);
    let tail = merged.get(origin.len()..).unwrap_or_default();
    let (path, query) = tail.split_once('?').unwrap_or((tail, ""));
    let mark = if tail.contains('?') { "?" } else { "" };

    format!("{origin}{}{mark}{query}", segments_settled(path))
}

fn refusal_read(answer: &mut http::Response<ureq::Body>) -> Vec<u8> {
    let mut refusal = Vec::new();
    let mut reader = answer.body_mut().as_reader().take(limits::ERROR_EXCERPT_BYTES_MAX);

    if let Err(cause) = reader.read_to_end(&mut refusal) {
        refusal.extend_from_slice(format!(" (the rest was lost: {cause})").as_bytes());
    }

    assert!(refusal.capacity() >= refusal.len(), "a refusal holds what it read");

    refusal
}

fn response_read(
    answer: &http::Response<ureq::Body>,
    body: Vec<u8>,
    streamed_count: u64,
) -> Response {
    let status = answer.status().as_u16();

    assert!((STATUS_MIN..=STATUS_MAX).contains(&status), "a read answer carries a real status");
    assert!(body.is_empty() || streamed_count == 0, "a body is held or streamed, never both");

    Response {
        body,
        is_near_limit: header_flag(answer, "x-ratelimit-nearlimit"),
        location: header_whole(answer, "location"),
        rate_limit_reason: header_text(answer, "ratelimit-reason"),
        retry_after_seconds: header_number(answer, "retry-after"),
        status,
        streamed_count,
    }
}

fn segments_settled(path: &str) -> String {
    assert!(path.len() < usize::MAX, "a path has a countable length");

    let segments: Vec<&str> = path.split('/').skip(1).collect();

    if !segments.iter().any(|segment| matches!(*segment, "." | "..")) {
        return path.to_owned();
    }

    let mut kept: Vec<&str> = Vec::with_capacity(segments.len());

    for segment in &segments {
        match *segment {
            "." => {}
            ".." => {
                let _dropped = kept.pop();
            }
            other => kept.push(other),
        }
    }

    if segments.last().is_some_and(|last| matches!(*last, "." | "..")) {
        kept.push("");
    }

    format!("/{}", kept.join("/"))
}

fn status_checked(
    response: http::Response<ureq::Body>,
) -> Result<http::Response<ureq::Body>, Error> {
    assert!(response.status().as_u16() >= STATUS_MIN, "a status has three digits");

    let status = response.status().as_u16();

    if status > STATUS_MAX {
        return Err(Error::Reply(format!("an impossible status {status}")));
    }

    assert!(status <= STATUS_MAX, "an answer carries a real status");

    Ok(response)
}

fn status_holds_file(status: u16, request: &Request) -> bool {
    assert!(status >= STATUS_MIN, "only a real status is judged");
    assert!(status <= STATUS_MAX, "only a real status is judged");

    if status == 206 {
        return request.headers().iter().any(|(name, _value)| name.eq_ignore_ascii_case("range"));
    }

    matches!(status, 200 | 203)
}

const fn status_holds_no_body(status: u16) -> bool {
    assert!(status >= STATUS_MIN, "the HTTP layer holds a status to three digits");

    matches!(status, 204 | 205)
}

const fn status_is_redirect(status: u16) -> bool {
    assert!(status >= STATUS_MIN, "the HTTP layer holds a status to three digits");

    matches!(status, 301 | 302 | 303 | 307 | 308)
}

fn timeout_of(request: &Request) -> Duration {
    let body_bytes = size::of_slice(request.body());
    let body_seconds = body_bytes.div_euclid(UPLOAD_BYTES_PER_SECOND_MIN);
    let seconds = REQUEST_TIMEOUT_SECONDS.strict_add(body_seconds).min(DOWNLOAD_TIMEOUT_SECONDS);

    assert!(seconds >= REQUEST_TIMEOUT_SECONDS, "a large body never shortens the timeout");
    assert!(seconds <= DOWNLOAD_TIMEOUT_SECONDS, "a timeout stays under the longest call");

    Duration::from_secs(seconds)
}

#[must_use]
pub(crate) fn url_is_sound(url: &str) -> bool {
    assert!(!url.is_empty(), "a request names where it goes");

    url.parse::<http::Uri>().is_ok()
}

#[cfg(test)]
mod tests {
    use core::time::Duration;

    use ureq::http;

    use super::{
        Agent,
        REQUEST_TIMEOUT_SECONDS,
        Reference,
        UPLOAD_BYTES_PER_SECOND_MIN,
        header_flag,
        header_number,
        header_text,
        header_whole,
        origin_of,
        reference_resolved,
        response_read,
        status_holds_file,
        status_is_redirect,
        timeout_of,
        url_is_sound,
    };
    use crate::limits;
    use crate::platform::CreateIssue;
    use crate::request::Request;
    use crate::request::tests::read_request;
    use crate::response::Response;
    use crate::size;

    #[test]
    fn a_header_that_cannot_be_read_as_its_kind_reads_as_absent() {
        let answer = answer_of(200, &[("retry-after", "soon"), ("ratelimit-reason", "")]);

        assert_eq!(header_number(&answer, "retry-after"), None, "a date is not a delay");
        assert_eq!(header_number(&answer, "missing"), None, "a missing header");
        assert_eq!(header_text(&answer, "ratelimit-reason"), None, "an empty reason");
        assert!(!header_flag(&answer, "x-ratelimit-nearlimit"), "a missing flag");
    }

    #[test]
    fn a_large_body_is_given_time_in_proportion_to_its_size() {
        let body = "x".repeat(size::offset(UPLOAD_BYTES_PER_SECOND_MIN.strict_mul(3)));
        let url = "https://api.atlassian.com/ex/jira/site/rest/api/3/issue".to_owned();
        let mut request = Request::of::<CreateIssue<'_>>(url, "Basic c2VjcmV0");

        assert_eq!(timeout_of(&request).as_secs(), REQUEST_TIMEOUT_SECONDS, "an empty body");

        request.body_text(&body).expect("a body");

        assert_eq!(timeout_of(&request).as_secs(), REQUEST_TIMEOUT_SECONDS.strict_add(3), "scaled");
        assert_eq!(UPLOAD_BYTES_PER_SECOND_MIN, 64 * 1024, "the floor rate is 64 KiB a second");
    }

    #[test]
    fn a_location_at_its_limit_is_kept_and_one_byte_more_is_not() {
        let limit = size::offset(u64::from(limits::URL_BYTES_MAX));
        let fits = "x".repeat(limit);
        let long = "x".repeat(limit.strict_add(1));
        let kept = answer_of(303, &[("location", fits.as_str())]);
        let cut = answer_of(303, &[("location", long.as_str())]);

        assert_eq!(header_whole(&kept, "location").as_deref(), Some(fits.as_str()), "at the limit");
        assert_eq!(header_whole(&cut, "location"), None, "past the limit");
    }

    #[test]
    fn a_location_is_kept_whole_or_not_at_all() {
        let long = format!("https://site/{}", "x".repeat(9_000));
        let answer = answer_of(303, &[("location", long.as_str())]);

        assert_eq!(header_whole(&answer, "location"), None, "a cut address would mislead");
        assert_eq!(header_whole(&answer, "missing"), None, "a missing header");
    }

    #[test]
    fn a_location_that_is_not_ascii_is_kept_rather_than_lost() {
        let raw = http::HeaderValue::from_bytes(b"https://site/caf\xc3\xa9").expect("a value");
        let builder = http::Response::builder().status(303).header("location", raw);
        let answer = builder.body(ureq::Body::builder().data("")).expect("a fixture answer");

        assert_eq!(
            header_whole(&answer, "location").as_deref(),
            Some("https://site/caf\u{e9}"),
            "the task address survives",
        );
    }

    #[test]
    fn a_new_agent_reads_a_refusal_rather_than_raising_it() {
        let agent = Agent::new();

        assert!(!agent.ureq.config().http_status_as_error(), "a 404 is an answer");
        assert!(agent.ureq.config().https_only(), "a redirect never leaves TLS");
        assert_eq!(agent.ureq.config().max_redirects(), 0, "ureq never follows a redirect itself");

        let idle = agent.ureq.config().timeout_per_read();

        assert_eq!(idle, Some(Duration::from_secs(60)), "a stall ends long before a download");
        assert_eq!(agent.ureq.config().max_idle_connections(), 16, "the wave keeps its pool");
    }

    #[test]
    fn a_rate_limited_answer_keeps_every_signal_jira_sends() {
        let answer = answer_of(
            429,
            &[
                ("retry-after", "31"),
                ("ratelimit-reason", "jira-burst-based"),
                ("x-ratelimit-nearlimit", "TRUE"),
            ],
        );

        let response = answer_read(&answer);

        assert_eq!(response.status, 429, "the status");
        assert_eq!(response.retry_after_seconds, Some(31), "the delay Jira asked for");
        assert_eq!(response.rate_limit_reason.as_deref(), Some("jira-burst-based"), "the reason");
        assert!(response.is_near_limit, "the flag reads in any case");
    }

    #[test]
    fn a_reason_is_cut_to_its_budget() {
        let long = "r".repeat(4_096);
        let answer = answer_of(429, &[("ratelimit-reason", long.as_str())]);
        let response = answer_read(&answer);

        assert_eq!(
            response.rate_limit_reason.map(|reason| reason.len()),
            Some(2_048),
            "the budget",
        );
    }

    #[test]
    fn a_redirect_answer_keeps_where_it_points() {
        let answer = answer_of(303, &[("location", "https://site/rest/api/3/task/10")]);
        let response = answer_read(&answer);

        assert_eq!(response.status, 303, "the status");
        assert_eq!(response.location.as_deref(), Some("https://site/rest/api/3/task/10"), "where");
    }

    #[test]
    fn a_redirect_is_only_what_names_another_address() {
        for status in [301, 302, 303, 307, 308] {
            assert!(status_is_redirect(status), "{status} moves");
        }

        for status in [200, 300, 304, 305, 306, 309, 399, 400] {
            assert!(!status_is_redirect(status), "{status} does not");
        }
    }

    #[test]
    fn a_reference_resolves_the_way_rfc_3986_says_it_does() {
        let base = "https://a/b/c/d;p?q";

        for (target, expected) in [
            ("g:h", "g:h"),
            ("g", "https://a/b/c/g"),
            ("g/", "https://a/b/c/g/"),
            ("/g", "https://a/g"),
            ("//g", "https://g"),
            ("?y", "https://a/b/c/d;p?y"),
            ("g?y", "https://a/b/c/g?y"),
            (";x", "https://a/b/c/;x"),
            ("g;x", "https://a/b/c/g;x"),
            ("", "https://a/b/c/d;p?q"),
            (".", "https://a/b/c/"),
            ("./", "https://a/b/c/"),
            ("./g", "https://a/b/c/g"),
            ("..", "https://a/b/"),
            ("../", "https://a/b/"),
            ("../g", "https://a/b/g"),
            ("../..", "https://a/"),
            ("../../", "https://a/"),
            ("../../g", "https://a/g"),
            ("../../../g", "https://a/g"),
            ("/./g", "https://a/g"),
            ("/../g", "https://a/g"),
            ("g.", "https://a/b/c/g."),
            (".g", "https://a/b/c/.g"),
            ("g..", "https://a/b/c/g.."),
            ("..g", "https://a/b/c/..g"),
            ("./../g", "https://a/b/g"),
            ("g/./h", "https://a/b/c/g/h"),
            ("g/../h", "https://a/b/c/h"),
            ("g;x=1/./y", "https://a/b/c/g;x=1/y"),
            ("g;x=1/../y", "https://a/b/c/y"),
            ("g?y/./x", "https://a/b/c/g?y/./x"),
            ("g?y/../x", "https://a/b/c/g?y/../x"),
        ] {
            let resolved = reference_resolved(Reference { base, target });

            assert_eq!(resolved, expected, "the reference {target}");
        }

        let shouted = Reference { base, target: "HTTPS://x/../y/z" };

        assert_eq!(reference_resolved(shouted), "HTTPS://x/y/z", "a dot never climbs into a host");

        for bare in ["https://a", "https://a?q"] {
            let resolved = reference_resolved(Reference { base: bare, target: "g" });

            assert_eq!(resolved, "https://a/g", "the base {bare}");
        }
    }

    #[test]
    fn a_status_holds_a_file_only_where_the_body_is_the_resource() {
        let plain = read_request();
        let mut ranged = read_request();

        ranged.header("Range", "bytes=0-6");

        for status in [200, 203] {
            assert!(status_holds_file(status, &plain), "{status} is the file");
        }

        for status in [201, 202, 204, 205, 206, 207, 303, 404] {
            assert!(!status_holds_file(status, &plain), "{status} is not");
        }

        assert!(status_holds_file(206, &ranged), "a part is the file only when it was asked for");
    }

    #[test]
    fn a_url_is_sound_when_the_http_layer_would_take_it() {
        assert!(url_is_sound("https://site/a?b=%7B%22c%22%7D&d=e"), "an escaped query");
        assert!(!url_is_sound("https://site/a?b=\"c\""), "a bare quote");
        assert!(!url_is_sound("https://site/a?b=<c>"), "bare angle brackets");
    }

    #[test]
    fn an_origin_is_the_scheme_and_authority_and_nothing_a_lookalike_can_borrow() {
        let origin = origin_of("https://api.atlassian.com/ex/jira/1");

        assert_eq!(origin, "https://api.atlassian.com", "the path is not part of it");
        assert_eq!(origin_of("https://api.atlassian.com?a=1"), origin, "and neither is a query");
        assert_eq!(origin_of("https://api.atlassian.com"), origin, "a bare origin is itself");
        assert_eq!(origin_of("HTTPS://x/y"), "HTTPS://x", "a scheme reads in any case");
        assert_ne!(origin_of("https://api.atlassian.com.evil.example/"), origin, "a suffix");
        assert_ne!(origin_of("https://api.atlassian.com@evil.example/"), origin, "a user part");
        assert_ne!(origin_of("https://api.atlassian.com:8443/"), origin, "another port");
    }

    #[test]
    #[should_panic(expected = "a header name is lowercase")]
    fn an_uppercase_header_name_is_a_programmer_error() {
        let answer = answer_of(200, &[]);
        let flag = header_flag(&answer, "X-RateLimit-NearLimit");

        assert!(!flag, "the panic above is the point of this test");
    }

    fn answer_of(status: u16, headers: &[(&str, &str)]) -> http::Response<ureq::Body> {
        assert!((100..=599).contains(&status), "a fixture answer carries a real status");
        assert!(headers.len() <= 8, "a fixture carries a few headers");

        let mut builder = http::Response::builder().status(status);

        for (name, value) in headers {
            builder = builder.header(*name, *value);
        }

        builder.body(ureq::Body::builder().data("")).expect("a fixture answer")
    }

    fn answer_read(answer: &http::Response<ureq::Body>) -> Response {
        assert!(answer.status().as_u16() >= 100, "a fixture answer carries a real status");
        assert!(answer.status().as_u16() <= 599, "a fixture answer carries a real status");

        response_read(answer, Vec::new(), 0)
    }
}
