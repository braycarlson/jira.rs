use core::fmt;

use serde_json::Value;

use crate::encoding;
use crate::error::Error;
use crate::limits;
use crate::method::Method;
use crate::multipart;
use crate::operation::Operation;
use crate::paging::Paging;
use crate::part::Part;
use crate::size;
use crate::upload::Upload;

const ACCEPT_ANY: &str = "*/*";
const ACCEPT_JSON: &str = "application/json";
const CONTENT_JSON: &str = "application/json";
const CONTENT_JSON_PATCH: &str = "application/json-patch+json";
const CONTENT_TEXT: &str = "text/plain";
const HEADER_COUNT_MAX: u64 = 16;
const HEADER_XSRF_NAME: &str = "X-Atlassian-Token";
const HEADER_XSRF_VALUE: &str = "no-check";
const _: () = assert!(HEADER_COUNT_MAX >= 2, "a request may carry the few headers Jira names");

pub struct Request {
    accept: &'static str,
    authorization: String,
    body: Vec<u8>,
    content_type: Option<String>,
    headers: Vec<(&'static str, String)>,
    is_read: bool,
    method: Method,
    url: String,
}

impl Request {
    #[must_use]
    pub fn accept(&self) -> &'static str {
        assert!(!self.accept.is_empty(), "a request names what it accepts");
        assert!(self.accept.contains('/'), "an accepted type is a media type");

        self.accept
    }

    pub(crate) fn accept_any(&mut self) {
        assert_eq!(self.accept, ACCEPT_JSON, "a request asks for one reply type");

        self.accept = ACCEPT_ANY;

        assert_eq!(self.accept, ACCEPT_ANY, "a download takes whatever Jira sends");
        assert_ne!(self.accept, ACCEPT_JSON, "a download asks for nothing narrower");
    }

    #[must_use]
    pub fn authorization(&self) -> &str {
        assert!(!self.authorization.is_empty(), "a request carries its credential");
        assert!(self.authorization.contains(' '), "an authorization names its scheme");

        self.authorization.as_str()
    }

    #[must_use]
    pub fn body(&self) -> &[u8] {
        assert!(size::of_slice(&self.body) <= limits::UPLOAD_BYTES_MAX, "a body fits its limit");

        if !self.body.is_empty() {
            assert!(self.content_type.is_some(), "a body names its type");
        }

        self.body.as_slice()
    }

    pub fn body_json(&mut self, body: &Value) -> Result<(), Error> {
        assert!(self.content_type.is_none(), "a request carries one body");

        let bytes = serde_json::to_vec(body).map_err(|cause| {
            Error::Input(format!("the request body cannot be written: {cause}"))
        })?;

        self.body_set(bytes, CONTENT_JSON.to_owned())
    }

    pub fn body_json_patch(&mut self, body: &Value) -> Result<(), Error> {
        assert!(self.content_type.is_none(), "a request carries one body");

        let bytes = serde_json::to_vec(body).map_err(|cause| {
            Error::Input(format!("the request body cannot be written: {cause}"))
        })?;

        self.body_set(bytes, CONTENT_JSON_PATCH.to_owned())
    }

    pub fn body_multipart(&mut self, parts: &[Part<'_>]) -> Result<(), Error> {
        assert!(self.content_type.is_none(), "a request carries one body");

        if parts.is_empty() {
            return Err(Error::Input("a multipart body holds no parts".to_owned()));
        }

        if parts.iter().any(|part| part.name.is_empty()) {
            return Err(Error::Input("a multipart part has no name".to_owned()));
        }

        let boundary = multipart::boundary(parts)?;
        let bytes = multipart::encode(parts, &boundary);

        self.body_set(bytes, format!("multipart/form-data; boundary={boundary}"))?;
        self.header(HEADER_XSRF_NAME, HEADER_XSRF_VALUE);

        Ok(())
    }

    fn body_set(&mut self, bytes: Vec<u8>, content_type: String) -> Result<(), Error> {
        assert!(!content_type.is_empty(), "a body names its type");
        assert!(self.content_type.is_none(), "a request carries one body");

        if !body_fits(size::of_slice(&bytes)) {
            return Err(Error::Input(format!(
                "a request body ran past the {} byte limit",
                limits::UPLOAD_BYTES_MAX,
            )));
        }

        self.body = bytes;
        self.content_type = Some(content_type);

        assert!(self.content_type.is_some(), "a body names its type");

        Ok(())
    }

    pub fn body_text(&mut self, text: &str) -> Result<(), Error> {
        assert!(self.content_type.is_none(), "a request carries one body");
        assert!(self.body.is_empty(), "a request carries one body");

        self.body_set(text.as_bytes().to_vec(), CONTENT_TEXT.to_owned())
    }

    pub fn body_upload(&mut self, upload: Upload<'_>) -> Result<(), Error> {
        assert!(self.content_type.is_none(), "a request carries one body");
        assert!(self.body.is_empty(), "a request carries one body");
        assert!(size::of_slice(upload.bytes) < u64::MAX, "an upload has a countable size");

        if upload.content_type.is_empty() {
            return Err(Error::Input("an upload names no content type".to_owned()));
        }

        if upload.content_type.chars().any(char::is_control) {
            return Err(Error::Input("an upload content type holds a control byte".to_owned()));
        }

        self.body_set(upload.bytes.to_vec(), upload.content_type.to_owned())
    }

    #[must_use]
    pub fn content_type(&self) -> Option<&str> {
        let found = self.content_type.as_deref();

        assert!(found.is_none_or(|text| !text.is_empty()), "a named type is never empty");

        found
    }

    pub(crate) fn content_type_settle(&mut self) {
        assert!(self.url.starts_with("https://"), "every request is encrypted");

        if self.method == Method::Get {
            return;
        }

        let named = self.content_type.get_or_insert_with(|| CONTENT_JSON.to_owned());

        assert!(!named.is_empty(), "a write names a type");
    }

    pub fn header(&mut self, name: &'static str, value: &str) {
        assert!(!name.is_empty(), "a header is named");
        assert!(size::of_slice(&self.headers) < HEADER_COUNT_MAX, "a request stays under its cap");

        self.headers.push((name, value.to_owned()));

        assert!(!self.headers.is_empty(), "a pushed header is held");
    }

    #[must_use]
    pub fn headers(&self) -> &[(&'static str, String)] {
        assert!(size::of_slice(&self.headers) <= HEADER_COUNT_MAX, "a request stays under its cap");

        self.headers.as_slice()
    }

    #[must_use]
    pub(crate) fn is_blank(&self) -> bool {
        assert!(!self.url.is_empty(), "a request names where it goes");
        assert!(self.headers.is_empty(), "a blank request carries no header yet");
        assert!(self.url.starts_with("https://"), "every request is encrypted");

        if self.url.contains('?') {
            return false;
        }

        self.body.is_empty()
    }

    #[must_use]
    pub(crate) fn is_complete(&self) -> bool {
        assert!(!self.url.is_empty(), "a request names where it goes");
        assert!(self.url.starts_with("https://"), "every request is encrypted");

        let (path, _query) = self.url.split_once('?').unwrap_or((self.url.as_str(), ""));

        !path.contains('{')
    }

    #[must_use]
    pub fn is_read(&self) -> bool {
        if self.method == Method::Get {
            assert!(self.is_read, "a GET is always a read");
        }

        self.is_read
    }

    #[must_use]
    pub fn method(&self) -> Method {
        if self.method == Method::Get {
            assert!(self.is_read, "a GET is always a read");
        }

        self.method
    }

    #[must_use]
    pub(crate) fn of<Called>(url: String, authorization: &str) -> Self
    where
        Called: Operation,
    {
        assert!(url.starts_with("https://"), "every request is encrypted");
        assert!(!authorization.is_empty(), "a request carries its credential");

        Self {
            accept: ACCEPT_JSON,
            authorization: authorization.to_owned(),
            body: Vec::new(),
            content_type: None,
            headers: Vec::new(),
            is_read: Called::IS_READ,
            method: Called::METHOD,
            url,
        }
    }

    pub fn path_number(&mut self, name: &'static str, value: i64) -> Result<(), Error> {
        assert!(!name.is_empty(), "a path parameter is named");
        assert!(self.url.contains(&format!("{{{name}}}")), "the path names {name}");

        self.path_text(name, &value.to_string())
    }

    pub fn path_text(&mut self, name: &'static str, value: &str) -> Result<(), Error> {
        assert!(!name.is_empty(), "a path parameter is named");

        if value.is_empty() {
            return Err(Error::Input(format!("the path parameter {name} is empty")));
        }

        if matches!(value, "." | "..") {
            return Err(Error::Input(format!("the path parameter {name} climbs the path")));
        }

        let placeholder = format!("{{{name}}}");

        assert!(self.url.contains(&placeholder), "the path names {name}");

        self.url = self.url.replace(&placeholder, &encoding::percent(value));

        assert!(!self.url.contains(&placeholder), "a filled parameter leaves no placeholder");
        assert!(!self.url.is_empty(), "a filled path still names where it goes");

        Ok(())
    }

    pub fn query_boolean(&mut self, name: &'static str, value: Option<bool>) {
        assert!(!name.is_empty(), "a query parameter is named");
        assert!(self.url.starts_with("https://"), "every request is encrypted");

        if let Some(found) = value {
            self.query_term(name, if found { "true" } else { "false" });
        }
    }

    pub(crate) fn query_merge(&mut self, paging: Paging, raw: &str) {
        assert!(!paging.cursor.is_empty(), "a walk names its cursor");
        assert!(!raw.is_empty(), "a next link names its terms");
        assert!(self.url.starts_with("https://"), "every request is encrypted");

        let terms: Vec<&str> = raw.split('&').filter(|term| !term.is_empty()).collect();
        let mut names: Vec<String> = terms.iter().map(|term| term_prefix(term)).collect();

        names.push(term_prefix(paging.cursor));

        let (path, query) = self.url.split_once('?').unwrap_or((self.url.as_str(), ""));
        let held = query.split('&').filter(|term| !term.is_empty());
        let kept = held.filter(|term| !names.iter().any(|name| term.starts_with(name.as_str())));
        let merged: Vec<&str> = kept.chain(terms.iter().copied()).collect();

        self.url = format!("{path}?{}", merged.join("&"));

        assert!(self.url.contains('?'), "a merged request holds a query");
    }

    pub fn query_number(&mut self, name: &'static str, value: Option<i64>) {
        assert!(!name.is_empty(), "a query parameter is named");
        assert!(self.url.starts_with("https://"), "every request is encrypted");

        if let Some(found) = value {
            self.query_term(name, &found.to_string());
        }
    }

    #[must_use]
    pub(crate) fn query_number_of(&self, name: &'static str) -> Option<u64> {
        assert!(!name.is_empty(), "a query parameter is named");
        assert!(self.url.starts_with("https://"), "every request is encrypted");

        self.query_of(name)?.parse().ok()
    }

    pub fn query_numbers(&mut self, name: &'static str, values: Option<&[i64]>) {
        assert!(!name.is_empty(), "a query parameter is named");
        assert!(self.url.starts_with("https://"), "every request is encrypted");

        for found in values.unwrap_or_default() {
            self.query_term(name, &found.to_string());
        }
    }

    #[must_use]
    pub fn query_of(&self, name: &str) -> Option<String> {
        assert!(!name.is_empty(), "a query parameter is named");
        assert!(self.url.starts_with("https://"), "every request is encrypted");

        let (_path, query) = self.url.split_once('?')?;
        let prefix = format!("{name}=");
        let term = query.split('&').find_map(|found| found.strip_prefix(prefix.as_str()))?;

        String::from_utf8(encoding::percent_decode(term)).ok()
    }

    fn query_pair(&mut self, (name, value): (&str, &str)) {
        assert!(!name.is_empty(), "a query parameter is named");

        self.url.push(if self.url.contains('?') { '&' } else { '?' });
        self.url.push_str(&encoding::percent(name));
        self.url.push('=');

        if !value.is_empty() {
            self.url.push_str(&encoding::percent(value));
        }

        assert!(self.url.contains('?'), "a term follows the query mark");
        assert!(!self.url.ends_with('&'), "a term follows its separator");
    }

    pub fn query_pairs(&mut self, pairs: &[(&str, &str)]) -> Result<(), Error> {
        assert!(self.url.starts_with("https://"), "every request is encrypted");

        if pairs.is_empty() {
            return Err(Error::Input("an operation by property names no property".to_owned()));
        }

        if pairs.iter().any(|(name, _value)| name.is_empty()) {
            return Err(Error::Input("a property has no name".to_owned()));
        }

        for pair in pairs {
            self.query_pair(*pair);
        }

        assert!(self.url.contains('?'), "a term follows the query mark");

        Ok(())
    }

    pub(crate) fn query_replace(&mut self, name: &'static str, value: &str) {
        assert!(!name.is_empty(), "a query parameter is named");
        assert!(!value.is_empty(), "a cursor is never empty");

        let prefix = format!("{name}=");

        if let Some((path, query)) = self.url.split_once('?') {
            let terms: Vec<&str> =
                query.split('&').filter(|term| !term.starts_with(&prefix)).collect();

            self.url = if terms.is_empty() {
                path.to_owned()
            } else {
                format!("{path}?{}", terms.join("&"))
            };
        }

        self.query_term(name, value);

        let query = self.url.split_once('?').map_or("", |(_path, found)| found);
        let held = query.split('&').filter(|term| term.starts_with(&prefix)).count();

        assert_eq!(held, 1, "a replaced term appears once");
    }

    fn query_term(&mut self, name: &'static str, value: &str) {
        assert!(!name.is_empty(), "a query parameter is named");

        self.query_pair((name, value));
    }

    pub fn query_text(&mut self, name: &'static str, value: Option<&str>) {
        assert!(!name.is_empty(), "a query parameter is named");
        assert!(self.url.starts_with("https://"), "every request is encrypted");

        if let Some(found) = value {
            self.query_term(name, found);
        }
    }

    pub fn query_texts(&mut self, name: &'static str, values: Option<&[&str]>) {
        assert!(!name.is_empty(), "a query parameter is named");
        assert!(self.url.starts_with("https://"), "every request is encrypted");

        for found in values.unwrap_or_default() {
            self.query_term(name, found);
        }
    }

    #[must_use]
    pub fn url(&self) -> &str {
        assert!(self.url.starts_with("https://"), "every request is encrypted");
        assert!(!self.url.contains(' '), "a URL holds no bare space");

        self.url.as_str()
    }

    pub(crate) fn url_check(&self) -> Result<(), Error> {
        assert!(self.is_complete(), "every path parameter was filled");

        if size::of_text(&self.url) > u64::from(limits::URL_BYTES_MAX) {
            return Err(Error::Input(format!(
                "a request URL ran past the {} byte limit",
                limits::URL_BYTES_MAX,
            )));
        }

        assert!(self.url.starts_with("https://"), "every request is encrypted");

        Ok(())
    }
}

impl fmt::Debug for Request {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Request")
            .field("method", &self.method)
            .field("url", &self.url)
            .finish_non_exhaustive()
    }
}

#[must_use]
fn body_fits(byte_count: u64) -> bool {
    assert!(usize::try_from(byte_count).is_ok(), "a body of {byte_count} bytes fits in memory");

    byte_count <= limits::UPLOAD_BYTES_MAX
}

fn term_prefix(term: &str) -> String {
    assert!(!term.is_empty(), "a query term is never empty");

    format!("{}=", term.split_once('=').map_or(term, |(name, _value)| name))
}

#[cfg(test)]
pub(crate) mod tests {
    use serde_json::json;

    use super::{Request, body_fits};
    use crate::base::Base;
    use crate::error::Error;
    use crate::limits;
    use crate::method::Method;
    use crate::operation::Operation;
    use crate::paging::{Next, Paging};
    use crate::part::Part;
    use crate::size;
    use crate::upload::Upload;

    const AUTHORIZATION: &str = "Basic secret";
    const LINK: Paging = Paging { cursor: "cursor", items: None, next: Next::Link("/next") };
    const URL: &str = "https://api.atlassian.com/ex/jira/site/rest/api/3/issue/{issueIdOrKey}";

    #[derive(Debug)]
    struct Fixture;

    #[derive(Debug)]
    struct FixtureWrite;

    impl Operation for Fixture {
        const BASE: Base = Base::Jira;
        const IS_READ: bool = true;
        const METHOD: Method = Method::Get;
        const PATH: &'static str = "/rest/api/3/issue/{issueIdOrKey}";

        type Options<'options> = ();
        type Reply = ();

        fn request(&self, _options: &(), request: &mut Request) -> Result<(), Error> {
            assert!(request.is_blank(), "a request starts blank");
            assert!(!request.url().is_empty(), "a request names where it goes");

            Ok(())
        }
    }

    impl Operation for FixtureWrite {
        const BASE: Base = Base::Jira;
        const IS_READ: bool = false;
        const METHOD: Method = Method::Post;
        const PATH: &'static str = "/rest/api/3/issue";

        type Options<'options> = ();
        type Reply = ();

        fn request(&self, _options: &(), request: &mut Request) -> Result<(), Error> {
            assert!(request.is_blank(), "a request starts blank");
            assert!(!request.url().is_empty(), "a request names where it goes");

            Ok(())
        }
    }

    #[test]
    fn a_bodiless_write_says_json_and_nothing_else_changes_its_type() {
        let mut read = blank_request();
        let mut upload = Request::of::<FixtureWrite>(URL.to_owned(), AUTHORIZATION);
        let mut write = Request::of::<FixtureWrite>(URL.to_owned(), AUTHORIZATION);

        upload.body_upload(Upload { bytes: b"\x89PNG", content_type: "image/png" }).expect("fits");

        read.content_type_settle();
        upload.content_type_settle();
        write.content_type_settle();

        assert_eq!(read.content_type(), None, "a GET names no type");
        assert_eq!(upload.content_type(), Some("image/png"), "a body keeps its own type");
        assert_eq!(write.content_type(), Some("application/json"), "a write says JSON");
        assert!(write.body().is_empty(), "the type adds no body");
    }

    #[test]
    fn a_body_a_caller_got_wrong_is_refused_rather_than_a_crash() {
        let blank = Upload { bytes: b"x", content_type: "" };
        let split = Upload { bytes: b"x", content_type: "image/png\r\nX-Injected: yes" };
        let unnamed = Part { bytes: b"x", content_type: "text/plain", file_name: None, name: "" };

        assert!(matches!(blank_request().body_upload(blank), Err(Error::Input(_))), "no type");
        assert!(matches!(blank_request().body_upload(split), Err(Error::Input(_))), "a new line");

        assert!(
            matches!(blank_request().body_multipart(&[unnamed]), Err(Error::Input(_))),
            "a part with no name",
        );
    }

    #[test]
    fn a_body_at_its_limit_fits_and_one_byte_more_does_not() {
        assert!(body_fits(0), "an empty body fits");
        assert!(body_fits(limits::UPLOAD_BYTES_MAX), "a body at the limit fits");
        assert!(!body_fits(limits::UPLOAD_BYTES_MAX.strict_add(1)), "one byte more does not");
    }

    #[test]
    fn a_body_declares_its_type_and_keeps_its_bytes() {
        let mut json_body = blank_request();
        let mut patch = blank_request();
        let mut text = blank_request();
        let mut upload = blank_request();

        json_body.body_json(&json!({"a": 1_i64})).expect("JSON writes");
        patch.body_json_patch(&json!([])).expect("JSON writes");
        text.body_text("word").expect("a text fits");
        upload.body_upload(Upload { bytes: b"\x89PNG", content_type: "image/png" }).expect("fits");

        assert_eq!(json_body.body(), b"{\"a\":1}", "the JSON bytes");
        assert_eq!(json_body.content_type(), Some("application/json"), "JSON");
        assert_eq!(patch.content_type(), Some("application/json-patch+json"), "a patch");
        assert_eq!(text.content_type(), Some("text/plain"), "a text");
        assert_eq!(upload.body(), b"\x89PNG", "the raw bytes");
        assert_eq!(upload.content_type(), Some("image/png"), "the upload's own type");
    }

    #[test]
    fn a_cursor_beside_a_longer_name_that_ends_alike_is_left_alone() {
        let mut request = blank_request();

        request.query_text("homepage", Some("x"));
        request.query_number("page", Some(1));
        request.query_replace("page", "2");

        assert!(request.url().ends_with("?homepage=x&page=2"), "only the cursor moved");
    }

    #[test]
    fn a_cursor_may_be_the_only_term() {
        let mut request = blank_request();

        request.query_number("startAt", Some(5));
        request.query_replace("startAt", "100");

        assert!(request.url().ends_with("{issueIdOrKey}?startAt=100"), "the old term is gone");
    }

    #[test]
    fn a_cursor_replaces_the_term_it_overrides_and_keeps_the_rest() {
        let mut request = blank_request();

        request.query_text("jql", Some("a"));
        request.query_number("startAt", Some(5));
        request.query_number("maxResults", Some(50));
        request.query_replace("startAt", "100");

        assert!(request.url().ends_with("?jql=a&maxResults=50&startAt=100"), "one cursor term");
    }

    #[test]
    fn a_multipart_body_declares_its_boundary_and_disarms_the_xsrf_check() {
        let mut request = blank_request();

        let part = Part {
            bytes: b"hello",
            content_type: "text/plain",
            file_name: Some("a.txt"),
            name: "file",
        };

        request.body_multipart(&[part]).expect("a part");

        let content_type = request.content_type().expect("a type");

        assert!(content_type.starts_with("multipart/form-data; boundary=jira-"), "the boundary");
        assert_eq!(request.headers(), [("X-Atlassian-Token", "no-check".to_owned())], "the header");
    }

    #[test]
    fn a_multipart_body_with_no_parts_is_refused() {
        let refused = blank_request().body_multipart(&[]);

        assert!(matches!(refused, Err(Error::Input(_))), "nothing to send");
    }

    #[test]
    fn a_next_link_gives_a_request_with_no_query_one() {
        let mut request = blank_request();

        request.query_merge(LINK, "cursor=a&&prev=false");

        assert!(request.url().ends_with("?cursor=a&prev=false"), "{}", request.url());
    }

    #[test]
    fn a_next_link_replaces_its_own_terms_and_leaves_the_rest_alone() {
        let mut request = blank_request();

        request.query_text("query", Some("two words"));
        request.query_number("limit", Some(5));
        request.query_merge(LINK, "limit=1&start=1&cursor=a%3D&prev=false");

        let url = request.url();

        assert!(url.contains("query=two%20words"), "a term the link does not name stays: {url}");
        assert_eq!(url.matches("limit=").count(), 1, "the link's term replaces it: {url}");
        assert!(url.contains("limit=1"), "with the value from the link: {url}");

        assert!(
            url.contains("start=1&cursor=a%3D&prev=false"),
            "the rest, as Jira wrote it: {url}",
        );

        assert_eq!(url.matches('?').count(), 1, "one query: {url}");
    }

    #[test]
    fn a_next_link_replaces_the_cursor_a_caller_started_from_even_under_another_name() {
        let mut request = blank_request();

        request.query_text("cursor", Some("start"));
        request.query_merge(LINK, "after=b");

        assert!(request.url().ends_with("?after=b"), "no stale cursor: {}", request.url());
    }

    #[test]
    fn a_path_value_is_escaped_so_it_cannot_leave_its_segment() {
        let mut request = blank_request();

        request.path_text("issueIdOrKey", "a/b?c=d").expect("a value");

        assert!(request.url().ends_with("/issue/a%2Fb%3Fc%3Dd"), "every separator is escaped");
        assert!(request.is_complete(), "no placeholder remains");
    }

    #[test]
    fn a_path_value_that_climbs_or_vanishes_is_refused() {
        for value in ["", ".", ".."] {
            let refused = blank_request().path_text("issueIdOrKey", value);

            assert!(matches!(refused, Err(Error::Input(_))), "{value:?} is refused");
        }
    }

    #[test]
    fn a_placeholder_lives_in_the_path_and_a_query_never_reads_as_one() {
        let mut request = read_request();

        request.query_merge(LINK, "c=%7B%7D");

        assert!(request.is_complete(), "an escaped brace is no placeholder");

        let braced = "https://api.atlassian.com/ex/jira/site/rest/api/3/myself?c={a}";

        assert!(Request::of::<Fixture>(braced.to_owned(), AUTHORIZATION).is_complete(), "a query");
        assert!(!blank_request().is_complete(), "a path that still holds one is incomplete");
    }

    #[test]
    fn a_property_term_is_named_by_its_caller_and_escaped_like_any_other() {
        let mut request = blank_request();

        request.query_pairs(&[("accountId", "a b"), ("env&x", "1")]).expect("two properties");

        assert!(request.url().ends_with("?accountId=a%20b&env%26x=1"), "{}", request.url());

        let none = blank_request().query_pairs(&[]);
        let blank = blank_request().query_pairs(&[("", "x")]);

        assert!(matches!(none, Err(Error::Input(_))), "Jira requires at least one");
        assert!(matches!(blank, Err(Error::Input(_))), "and each one names itself");
    }

    #[test]
    fn a_query_term_appears_only_when_it_is_set() {
        let mut request = blank_request();

        request.query_boolean("a", None);
        request.query_number("b", None);
        request.query_numbers("c", None);
        request.query_text("d", None);
        request.query_texts("e", None);

        assert!(request.is_blank(), "nothing was set");

        request.query_boolean("a", Some(true));
        request.query_number("b", Some(-1));
        request.query_numbers("c", Some(&[1, 2]));
        request.query_text("d", Some("x y"));
        request.query_texts("e", Some(&["p", "q"]));

        assert!(
            request.url().ends_with("?a=true&b=-1&c=1&c=2&d=x%20y&e=p&e=q"),
            "each term is escaped and a list repeats its name",
        );
    }

    #[test]
    fn a_query_term_is_named_exactly_and_a_missing_one_reads_as_absent() {
        let mut request = blank_request();

        assert_eq!(request.query_of("page"), None, "a request with no query holds no term");

        request.query_text("homepage", Some("x"));

        assert_eq!(request.query_of("page"), None, "a longer name that ends alike is no match");
        assert_eq!(request.query_of("homepage").as_deref(), Some("x"), "the whole name matches");
    }

    #[test]
    fn a_query_term_reads_back_as_it_was_written_whatever_it_holds() {
        for value in ["plain", "a b&c=d", "*all", "project = \"ABC\"", "caf\u{e9}+%2F", "x,y"] {
            let mut request = blank_request();

            request.query_text("before", Some("1"));
            request.query_text("jql", Some(value));
            request.query_text("after", Some("2"));

            assert_eq!(request.query_of("jql").as_deref(), Some(value), "the term {value}");
        }
    }

    #[test]
    fn a_term_or_a_body_ends_a_blank_request() {
        let mut termed = blank_request();
        let mut bodied = blank_request();

        assert!(termed.is_blank(), "a fresh request holds nothing");

        termed.query_text("a", Some("b"));
        bodied.body_json(&json!({"a": "b"})).expect("a body");

        assert!(!termed.is_blank(), "a term has landed");
        assert!(!bodied.is_blank(), "a body has landed");
    }

    #[test]
    fn a_url_at_its_limit_passes_and_one_byte_more_is_refused() {
        let base = "https://api.atlassian.com/";
        let room = size::offset(u64::from(limits::URL_BYTES_MAX)).strict_sub(base.len());
        let fits = format!("{base}{}", "x".repeat(room));
        let long = format!("{fits}x");

        assert!(Request::of::<Fixture>(fits, AUTHORIZATION).url_check().is_ok(), "at the limit");
        assert!(Request::of::<Fixture>(long, AUTHORIZATION).url_check().is_err(), "past it");
    }

    #[test]
    fn an_empty_text_is_a_term_rather_than_a_missing_one() {
        let mut request = blank_request();

        request.query_text("jql", Some(""));

        assert!(request.url().ends_with("?jql="), "an empty value is still sent");
    }

    fn blank_request() -> Request {
        let request = Request::of::<Fixture>(URL.to_owned(), AUTHORIZATION);

        assert!(request.is_blank(), "a fixture starts blank");
        assert!(!request.is_complete(), "a fixture holds a placeholder to fill");

        request
    }

    #[test]
    #[should_panic(expected = "the path names unknown")]
    fn filling_a_parameter_the_path_never_named_is_a_programmer_error() {
        let filled = blank_request().path_text("unknown", "x");

        assert!(filled.is_err(), "the panic above is the point of this test");
    }

    pub(crate) fn read_request() -> Request {
        let url = "https://api.atlassian.com/ex/jira/site/rest/api/3/myself".to_owned();
        let request = Request::of::<Fixture>(url, AUTHORIZATION);

        assert!(request.is_read(), "a fixture read is a read");
        assert!(request.is_complete(), "a fixture read has nothing to fill");

        request
    }

    #[test]
    fn the_debug_form_never_prints_the_credential() {
        let printed = format!("{:?}", blank_request());

        assert!(printed.contains("/rest/api/3/issue/"), "the URL is printed");
        assert!(!printed.contains("secret"), "the credential is not");
    }

    pub(crate) fn write_request() -> Request {
        let url = "https://api.atlassian.com/ex/jira/site/rest/api/3/issue".to_owned();
        let request = Request::of::<FixtureWrite>(url, AUTHORIZATION);

        assert!(!request.is_read(), "a fixture write is a write");
        assert!(request.is_complete(), "a fixture write has nothing to fill");

        request
    }
}
