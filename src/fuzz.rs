use core::iter;
use core::time::Duration;
use std::io::{Cursor, Seek, Write};
use std::sync::Mutex;

use serde_json::{Map, Value, json};

use crate::adf;
use crate::agent;
use crate::assets::AQLFindObjects;
use crate::client::Client;
use crate::credentials::Credentials;
use crate::encoding::{percent_decode, query_escape};
use crate::error::Error;
use crate::kind::{Any, Nothing, Text};
use crate::limits;
use crate::operation::Operation;
use crate::page::Page;
use crate::paged::Paged;
use crate::paging::{Next, Paging};
use crate::platform::{
    CreateIssue,
    GetAllUsersDefault,
    GetAttachmentContent,
    GetComment,
    GetIDsOfWorklogsModifiedSince,
    GetServerInfo,
    SearchAndReconsileIssuesUsingJQL,
    SearchProjects,
    get_comment,
};
use crate::prng::{Prng, Seed};
use crate::reply::Reply;
use crate::request::Request;
use crate::response::Response;
use crate::service_desk::GetArticles;
use crate::size;
use crate::text;
use crate::transport::Transport;
use crate::wait::Wait;

pub const CASE_COUNT_SMOKE: u32 = 1_000;
pub const NAMES: [&str; 8] = ["adf", "client", "link", "page", "reply", "text", "url", "walk"];
pub const SEED_SMOKE: u64 = 123;
const ATTEMPT_COUNT_MAX: u64 = 5;

const CREDENTIALS: Credentials<'static> =
    Credentials::Token { cloud_id: "fuzz-site", email: "fuzz@example.com", token: "fuzz" };

const DEPTH_MAX: u32 = 10;
const DOWNLOAD_FILE: &[u8] = b"the whole of the file";
const DOWNLOAD_STALE: &[u8] = b"what a cut attempt left in the sink, which runs longer";
const FAULT_BODIES: [&[u8]; 6] = [b"{}", b"", b"<html>", b"\xff", b"\"\"", b"null"];
const FAULT_STATUSES: [u16; 11] = [200, 201, 204, 303, 400, 401, 404, 429, 429, 500, 503];

const FLATTEN_BUDGET: adf::Budget =
    adf::Budget { node_count_max: 20_000, text_bytes_max: 256 * 1024 };

const GROUP_COUNT_MAX: u64 = 4;
const LEAF_COUNT_MAX: u64 = 12;
const LINK_PIECE_COUNT_MAX: u64 = 12;

const LINK_PIECES: [&str; 27] = [
    "https://evil.example/elsewhere",
    "{",
    "}",
    "\"",
    "<",
    ">",
    "\\",
    "|",
    "^",
    "`",
    "?",
    "&",
    "=",
    "#",
    "cursor=",
    "cursor=a%3D",
    "start=1",
    "%",
    "%0d%0a",
    " ",
    "\r\n",
    "\t",
    "\u{e9}",
    "\u{7f}",
    "..",
    "/",
    "@",
];

const MARKER: &str = "MARK7F3A";

const PAGE_FIELDS: [&str; 8] =
    ["values", "isLast", "total", "nextPageToken", "issues", "last", "lastPage", "x"];

const PAGE_FIELD_COUNT_MAX: u64 = 8;
const PAGE_FLAGS: [&str; 4] = ["isLast", "isLastPage", "last", "lastPage"];
const PAGE_HOSTILE_COUNT_MAX: u64 = 12;

const PAGINGS: [Paging; 5] = [
    Paging { cursor: "startAt", items: None, next: Next::Offset },
    Paging { cursor: "startAt", items: Some("/values"), next: Next::Offset },
    Paging { cursor: "page", items: Some("/values"), next: Next::Number },
    Paging { cursor: "t", items: Some("/issues"), next: Next::Token("/nextPageToken") },
    Paging { cursor: "cursor", items: Some("/values"), next: Next::Link("/nextPageToken") },
];

const REPLY_DEPTH_MAX: u64 = 400;
const REPLY_PIECE_COUNT_MAX: u64 = 12;

const REPLY_PIECES: [&[u8]; 22] = [
    b"\"",
    b"\\",
    b"\\u",
    b"d83d",
    b"\\ud83d",
    b"\\ude00",
    b"\xff",
    b"\xc3",
    b" ",
    b"\n",
    b"{",
    b"}",
    b"[",
    b"]",
    b"null",
    b"true",
    b"1",
    b",",
    b":",
    b"<html>",
    b"a",
    "\u{e9}".as_bytes(),
];

const REPLY_STATUSES: [u16; 5] = [200, 201, 204, 207, 303];
const RETRY_AFTER_SECONDS_GUARD: u64 = 900;
const URL_BASE: &str = "https://api.atlassian.com/ex/jira/site";
const URL_SITE: &str = "https://api.atlassian.com/ex/jira/fuzz-site/rest/";

const URL_PIECES: [&str; 14] =
    ["/", "..", ".", "?", "&", "=", "#", "%", "%2F", "{id}", " ", "\u{e9}", "\u{1f600}", "a"];

const URL_PIECE_COUNT_MAX: u64 = 8;
const WALK_ITEM_COUNT_MAX: u64 = 40;
const WALK_PAGE_ITEM_COUNT_MAX: u64 = 8;

const TYPES_BLOCK: [&str; 10] = [
    "paragraph",
    "bulletList",
    "listItem",
    "panel",
    "blockquote",
    "tableRow",
    "taskList",
    "taskItem",
    "expand",
    "media",
];

const TYPES_HOSTILE: [&str; 4] = ["doc", "somethingNew", "mediaSingle", "codeBlock"];
const WIDE_BYTES: u64 = FLATTEN_BUDGET.text_bytes_max.div_euclid(3);
const WIDE_PERCENT: u64 = 4;

const WORD_ALPHABET: [char; 12] =
    ['a', 'B', '9', '_', '-', '.', ' ', '\n', '\t', '\u{e9}', '\u{4e16}', '\u{1f600}'];

const WORD_CHARACTER_COUNT_MAX: u64 = 24;
const _: () = assert!(ATTEMPT_COUNT_MAX >= 2, "a failure is retried at least once");
const _: () = assert!(PAGE_HOSTILE_COUNT_MAX >= 1, "a walk reads at least one hostile page");
const _: () = assert!(CASE_COUNT_SMOKE >= 1, "a smoke run checks at least one case");
const _: () = assert!(DEPTH_MAX >= 1, "a tree folds at least once");
const _: () = assert!(GROUP_COUNT_MAX >= 1, "a fold groups at least one child");
const _: () = assert!(LEAF_COUNT_MAX >= 1, "a document holds at least one leaf");
const _: () = assert!(WIDE_PERCENT <= 100, "a chance is a percentage");
const _: () = assert!(WORD_CHARACTER_COUNT_MAX >= 1, "a word holds at least one character");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Attempted {
    Read,
    Write,
}

#[derive(Clone, Copy, Debug)]
enum Fault {
    Answer {
        body: &'static [u8],
        is_near_limit: bool,
        retry_after_seconds: Option<u64>,
        status: u16,
    },
    Dropped,
}

#[derive(Debug)]
struct Faulty {
    answered_count: Mutex<u64>,
    faults: Vec<Fault>,
    waits: Mutex<Vec<Duration>>,
}

#[derive(Debug)]
struct Hostile {
    bodies: Mutex<Vec<Value>>,
}

#[derive(Clone, Copy, Debug)]
struct Model {
    item_count: u64,
    page_item_count: u64,
    style: Style,
}

#[derive(Debug)]
struct Modeled {
    model: Model,
    request_count: Mutex<u64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Style {
    Bare,
    Flag,
    Link,
    Number,
    Since,
    Token,
    Total,
}

impl Faulty {
    fn fault_next(&self) -> Result<Fault, Error> {
        let mut answered = self
            .answered_count
            .lock()
            .map_err(|_poisoned| Error::Input("the attempt count is poisoned".to_owned()))?;

        let fault = self.faults.get(size::offset(*answered)).copied().unwrap_or(Fault::Dropped);

        *answered = answered.strict_add(1);
        drop(answered);

        Ok(fault)
    }
}

impl Model {
    fn page_of(self, start: u64) -> Value {
        assert!(start <= self.item_count, "a walk asks for a page inside the list");

        let end = start.strict_add(self.page_item_count).min(self.item_count);
        let items: Vec<u64> = (start..end).collect();
        let is_last = end >= self.item_count;

        assert!(end <= self.item_count, "a page stays inside the list");
        assert!(size::of_slice(&items) <= self.page_item_count, "a page holds what it may");

        match self.style {
            Style::Bare => json!(items),
            Style::Flag => json!({"isLast": is_last, "values": items}),
            Style::Link => {
                let next = (!is_last).then(|| format!("https://x/y?limit=1&cursor={end}"));

                json!({"_links": {"next": next}, "values": items})
            }
            Style::Number => json!({"isLast": is_last, "objectEntries": items}),
            Style::Since => {
                let next = (!is_last).then(|| format!("https://x/y?since={end}"));

                json!({"lastPage": is_last, "nextPage": next, "values": items})
            }
            Style::Token => {
                let next = (!is_last).then(|| end.to_string());

                json!({"issues": items, "nextPageToken": next})
            }
            Style::Total => json!({"total": self.item_count, "values": items}),
        }
    }

    fn request_count_expected(self) -> u64 {
        let page_count = self.item_count.div_ceil(self.page_item_count);

        if self.style == Style::Bare {
            return page_count.strict_add(1);
        }

        page_count.max(1)
    }

    fn start_of(self, request: &Request) -> u64 {
        assert!(self.page_item_count >= 1, "a page holds at least one item");

        match self.style {
            Style::Bare | Style::Flag | Style::Total => term_of(request, "startAt").unwrap_or(0),
            Style::Link => term_of(request, "cursor").unwrap_or(0),
            Style::Number => {
                let page = term_of(request, "page").unwrap_or(1);

                page.strict_sub(1).strict_mul(self.page_item_count)
            }
            Style::Since => term_of(request, "since").unwrap_or(0),
            Style::Token => term_of(request, "nextPageToken").unwrap_or(0),
        }
    }

    const fn styled(self, style: Style) -> Self {
        Self { item_count: self.item_count, page_item_count: self.page_item_count, style }
    }
}

impl Transport for Faulty {
    fn fetch(&self, request: &Request, bytes_max: u64) -> Result<Response, Error> {
        assert!(bytes_max >= 1, "a body may hold at least one byte");
        assert!(request.url().starts_with("https://"), "every request is encrypted");

        match self.fault_next()? {
            Fault::Answer { body, is_near_limit, retry_after_seconds, status } => Ok(Response {
                body: body.to_vec(),
                is_near_limit,
                location: None,
                rate_limit_reason: None,
                retry_after_seconds,
                status,
                streamed_count: 0,
            }),
            Fault::Dropped => Err(Error::Transport("the fuzzer dropped the link".to_owned())),
        }
    }

    fn stream<Sink>(&self, request: &Request, sink: &mut Sink) -> Result<Response, Error>
    where
        Sink: Seek + Write,
    {
        assert!(request.is_read(), "a download is a read");

        let fault = self.fault_next()?;
        let status_read = fault_status(fault);
        let is_landed = status_read.is_some_and(|found| (200..=299).contains(&found));

        if status_read.is_none_or(|found| found < 300) {
            let bytes = if is_landed { DOWNLOAD_FILE } else { DOWNLOAD_STALE };

            sink.write_all(bytes).map_err(|cause| Error::Input(cause.to_string()))?;
        }

        let Fault::Answer { body, is_near_limit, retry_after_seconds, status } = fault else {
            return Err(Error::Transport("the fuzzer dropped the link".to_owned()));
        };

        Ok(Response {
            body: if is_landed { Vec::new() } else { body.to_vec() },
            is_near_limit,
            location: None,
            rate_limit_reason: None,
            retry_after_seconds,
            status,
            streamed_count: if is_landed { size::of_slice(DOWNLOAD_FILE) } else { 0 },
        })
    }

    fn wait(&self, wait: &Wait<'_>) -> Result<(), Error> {
        assert!(!wait.reason.is_empty(), "a wait always says why");

        let mut waits = self
            .waits
            .lock()
            .map_err(|_poisoned| Error::Input("the wait log is poisoned".to_owned()))?;

        waits.push(wait.delay);
        drop(waits);

        Ok(())
    }
}

impl Transport for Hostile {
    fn fetch(&self, request: &Request, bytes_max: u64) -> Result<Response, Error> {
        let url = request.url();
        let (path, _query) = url.split_once('?').unwrap_or((url, ""));

        assert!(bytes_max >= 1, "a body may hold at least one byte");
        assert!(request.is_read(), "a walk only ever reads");
        assert!(path.starts_with(URL_SITE), "a walk stays on its own site: {url}");
        assert!(url.bytes().all(|byte| byte.is_ascii_graphic()), "only URL bytes: {url}");
        assert!(!url.contains('#'), "a fragment never reaches the wire: {url}");
        assert!(agent::url_is_sound(url), "the HTTP layer takes it: {url}");

        let mut bodies = self
            .bodies
            .lock()
            .map_err(|_poisoned| Error::Input("the pages are poisoned".to_owned()))?;

        let body = if bodies.is_empty() {
            json!({"values": [], "issues": [], "isLast": true})
        } else {
            bodies.remove(0)
        };

        drop(bodies);

        Ok(Response {
            body: serde_json::to_vec(&body).unwrap_or_default(),
            is_near_limit: false,
            location: None,
            rate_limit_reason: None,
            retry_after_seconds: None,
            status: 200,
            streamed_count: 0,
        })
    }

    fn stream<Sink>(&self, request: &Request, _sink: &mut Sink) -> Result<Response, Error>
    where
        Sink: Seek + Write,
    {
        assert!(request.is_read(), "a download is a read");

        Err(Error::Input("a walk downloads nothing".to_owned()))
    }

    fn wait(&self, wait: &Wait<'_>) -> Result<(), Error> {
        assert!(!wait.reason.is_empty(), "a wait always says why");
        assert!(wait.delay.as_millis() <= u128::from(limits::WAIT_MILLIS_MAX), "a wait fits");

        Ok(())
    }
}

impl Transport for Modeled {
    fn fetch(&self, request: &Request, bytes_max: u64) -> Result<Response, Error> {
        assert!(bytes_max >= 1, "a body may hold at least one byte");
        assert!(request.is_read(), "a walk only ever reads");

        let start = self.model.start_of(request);
        let page = self.model.page_of(start);

        let mut request_count = self
            .request_count
            .lock()
            .map_err(|_poisoned| Error::Input("the request count is poisoned".to_owned()))?;

        *request_count = request_count.strict_add(1);
        drop(request_count);

        Ok(Response {
            body: serde_json::to_vec(&page).unwrap_or_default(),
            is_near_limit: false,
            location: None,
            rate_limit_reason: None,
            retry_after_seconds: None,
            status: 200,
            streamed_count: 0,
        })
    }

    fn stream<Sink>(&self, request: &Request, _sink: &mut Sink) -> Result<Response, Error>
    where
        Sink: Seek + Write,
    {
        assert!(request.is_read(), "a download is a read");

        Err(Error::Input("a walk downloads nothing".to_owned()))
    }

    fn wait(&self, wait: &Wait<'_>) -> Result<(), Error> {
        assert!(!wait.reason.is_empty(), "a wait always says why");
        assert!(wait.delay.as_millis() <= u128::from(limits::WAIT_MILLIS_MAX), "a wait fits");

        Ok(())
    }
}

#[must_use]
pub fn adf_flatten(seed: u64, case_count_max: u32) -> u32 {
    assert!(case_count_max >= 1, "seed {seed}: a run checks at least one case");

    let mut prng = Prng::from_seed(seed);
    let mut case_count: u32 = 0;
    let bytes_max = FLATTEN_BUDGET.text_bytes_bound();

    for _case_index in 0..case_count_max {
        let is_planted = prng.boolean();

        let document =
            if is_planted { document_planted(&mut prng) } else { document_hostile(&mut prng) };

        let flattened = adf::text_read_within(&document, FLATTEN_BUDGET);
        let is_truncated = flattened.ends_with(adf::TRUNCATION_NOTE);
        let is_marker_due = is_planted && !is_truncated;

        assert!(
            size::of_text(&flattened) <= bytes_max,
            "seed {seed} case {case_count}: the flattened text passed its budget",
        );

        assert!(
            !flattened.starts_with(char::is_whitespace),
            "seed {seed} case {case_count}: the flattened text opens with whitespace",
        );

        if is_marker_due {
            assert!(
                flattened.contains(MARKER),
                "seed {seed} case {case_count}: the planted marker did not survive",
            );
        }

        case_count = case_count.strict_add(1);
    }

    assert_eq!(case_count, case_count_max, "every case ran");

    case_count
}

fn attempt_count_expected(faults: &[Fault], attempted: Attempted) -> u64 {
    assert_eq!(size::of_slice(faults), ATTEMPT_COUNT_MAX, "a script answers every attempt");

    let retried = faults
        .iter()
        .copied()
        .take(size::offset(ATTEMPT_COUNT_MAX.strict_sub(1)))
        .take_while(|fault| fault_is_retried(*fault, attempted));

    let attempt_count = size::of_offset(retried.count()).strict_add(1);

    assert!(attempt_count <= ATTEMPT_COUNT_MAX, "a failure is retried at most four times");

    attempt_count
}

#[must_use]
pub fn client_retry(seed: u64, case_count_max: u32) -> u32 {
    assert!(case_count_max >= 1, "seed {seed}: a run checks at least one case");

    let mut prng = Prng::from_seed(seed);
    let mut case_count: u32 = 0;

    for _case_index in 0..case_count_max {
        let faults: Vec<Fault> = iter::repeat_with(|| fault_draw(&mut prng))
            .take(size::offset(ATTEMPT_COUNT_MAX))
            .collect();

        let attempted = if prng.boolean() { Attempted::Write } else { Attempted::Read };

        retry_run(seed, &faults, attempted);
        download_run(seed, &faults);
        walk_run(seed, &mut prng);

        case_count = case_count.strict_add(1);
    }

    assert_eq!(case_count, case_count_max, "every case ran");

    case_count
}

fn document_hostile(prng: &mut Prng) -> Value {
    let leaf_count = prng.index(LEAF_COUNT_MAX).strict_add(1);
    let mut level: Vec<Value> = Vec::with_capacity(size::offset(leaf_count));

    for _leaf_index in 0..leaf_count {
        level.push(node_hostile(prng));
    }

    assert_eq!(size::of_slice(&level), leaf_count, "every leaf is drawn");

    json!({"type": "doc", "content": tree_fold(prng, level, &TYPES_HOSTILE)})
}

fn document_planted(prng: &mut Prng) -> Value {
    let leaf_count = prng.index(LEAF_COUNT_MAX).strict_add(1);
    let marker_index = prng.index(leaf_count);
    let mut level: Vec<Value> = Vec::with_capacity(size::offset(leaf_count));

    assert!(marker_index < leaf_count, "the marker sits on one of the leaves");

    for leaf_index in 0..leaf_count {
        if leaf_index == marker_index {
            level.push(json!({"type": "text", "text": MARKER}));

            continue;
        }

        level.push(json!({"type": "text", "text": word_build(prng)}));
    }

    assert_eq!(size::of_slice(&level), leaf_count, "every leaf is drawn");

    json!({"type": "doc", "content": tree_fold(prng, level, &TYPES_BLOCK)})
}

fn download_run(seed: Seed, faults: &[Fault]) {
    assert!(!faults.is_empty(), "seed {seed}: a script answers at least once");

    let network = Faulty {
        answered_count: Mutex::new(0),
        faults: faults.to_vec(),
        waits: Mutex::new(Vec::new()),
    };

    let built = Client::new(network, &CREDENTIALS);

    assert!(built.is_ok(), "seed {seed}: the fixture credentials are sound");

    let Ok(client) = built else {
        return;
    };

    let mut sink = Cursor::new(Vec::new());
    let outcome = client.download(&GetAttachmentContent { id: "1" }, &mut sink);

    retry_check(seed, faults, Attempted::Read, client.network());

    let attempt_expected = attempt_count_expected(faults, Attempted::Read);
    let last = faults.get(size::offset(attempt_expected.strict_sub(1))).copied();
    let status = last.and_then(fault_status);
    let is_file = status.is_some_and(|found| (200..=299).contains(&found));

    assert_eq!(outcome.is_ok(), is_file, "seed {seed}: {faults:?} read as {outcome:?}");

    if is_file {
        let held = sink.get_ref().get(..DOWNLOAD_FILE.len()).unwrap_or_default();

        assert_eq!(held, DOWNLOAD_FILE, "seed {seed}: the last attempt starts at the start");
        assert_eq!(outcome.ok(), Some(size::of_slice(DOWNLOAD_FILE)), "seed {seed}: the count");
    }
}

fn fault_draw(prng: &mut Prng) -> Fault {
    if prng.chance(25) {
        return Fault::Dropped;
    }

    let status = pick(prng, &FAULT_STATUSES);

    let retry_after_seconds = match prng.index(4) {
        0 => Some(prng.index(RETRY_AFTER_SECONDS_GUARD.strict_add(1))),
        1 => Some(RETRY_AFTER_SECONDS_GUARD.strict_add(prng.index(1_000))),
        _ => None,
    };

    let body = pick(prng, &FAULT_BODIES);

    Fault::Answer { body, is_near_limit: prng.chance(20), retry_after_seconds, status }
}

fn fault_is_retried(fault: Fault, attempted: Attempted) -> bool {
    let Fault::Answer { body: _, is_near_limit: _, retry_after_seconds, status } = fault else {
        return attempted == Attempted::Read;
    };

    assert!((100..=599).contains(&status), "a scripted answer carries a real status");

    if !fault_status_is_retried(status, attempted) {
        return false;
    }

    retry_after_seconds.is_none_or(|seconds| seconds <= RETRY_AFTER_SECONDS_GUARD)
}

fn fault_outcome_check(
    seed: Seed,
    fault: Fault,
    attempted: Attempted,
    outcome: &Result<(), Error>,
) {
    let Fault::Answer { body, is_near_limit: _, retry_after_seconds: _, status } = fault else {
        let is_expected = match attempted {
            Attempted::Read => matches!(outcome, Err(Error::Transport(_))),
            Attempted::Write => matches!(outcome, Err(Error::Indefinite(_))),
        };

        assert!(is_expected, "seed {seed}: a dropped {attempted:?} read as {outcome:?}");

        return;
    };

    if status_is_success(status, attempted) {
        if reply_is_unreadable(body, attempted) {
            let is_fault = matches!(outcome, Err(Error::Reply(_)));

            assert!(is_fault, "seed {seed}: {body:?} on a read came back as {outcome:?}");

            return;
        }

        assert!(outcome.is_ok(), "seed {seed}: {status} read as {outcome:?}");

        return;
    }

    if status_may_have_landed(status, attempted) {
        let is_indefinite = matches!(outcome, Err(Error::Indefinite(_)));

        assert!(is_indefinite, "seed {seed}: a {status} on a write read as {outcome:?}");

        return;
    }

    let found = outcome.as_ref().err().and_then(Error::status);

    assert_eq!(found, Some(status), "seed {seed}: {status} read as {outcome:?}");
}

const fn fault_status(fault: Fault) -> Option<u16> {
    match fault {
        Fault::Answer { body: _, is_near_limit: _, retry_after_seconds: _, status } => Some(status),
        Fault::Dropped => None,
    }
}

const fn fault_status_is_retried(status: u16, attempted: Attempted) -> bool {
    assert!(status >= 100, "a scripted answer carries a real status");

    if status == 429 {
        return true;
    }

    if status < 500 {
        return false;
    }

    matches!(attempted, Attempted::Read)
}

fn link_build(prng: &mut Prng) -> String {
    let piece_count = prng.index(LINK_PIECE_COUNT_MAX).strict_add(1);
    let mut link = String::new();

    for _piece_index in 0..piece_count {
        link.push_str(pick(prng, &LINK_PIECES));
    }

    assert!(!link.is_empty(), "a link holds at least one piece");
    assert!(piece_count <= LINK_PIECE_COUNT_MAX, "a link holds at most its piece budget");

    link
}

fn link_escaped(reply: &Value, pointer: &str) -> Option<(String, u32)> {
    assert!(pointer.starts_with('/'), "a next link is named by a pointer");

    let link = reply.pointer(pointer)?.as_str()?;
    let (_address, rest) = link.split_once('?')?;
    let query = rest.split('#').next().filter(|text| !text.is_empty())?;

    Some((query_escape(query), limits::URL_BYTES_MAX))
}

fn link_page(prng: &mut Prng) -> Value {
    let link = link_build(prng);
    let is_last = prng.chance(WIDE_PERCENT);
    let word = word_build(prng);
    let page = json!({"_links": {"next": link}, "isLastPage": is_last, "values": [word]});

    assert!(page.is_object(), "a page is an object");

    page
}

#[must_use]
pub fn link_walk(seed: u64, case_count_max: u32) -> u32 {
    assert!(case_count_max >= 1, "seed {seed}: a run checks at least one case");

    let mut prng = Prng::from_seed(seed);
    let mut case_count: u32 = 0;

    for _case_index in 0..case_count_max {
        let body_count = prng.index(PAGE_HOSTILE_COUNT_MAX).strict_add(1);

        let bodies: Vec<Value> =
            iter::repeat_with(|| link_page(&mut prng)).take(size::offset(body_count)).collect();

        assert_eq!(size::of_slice(&bodies), body_count, "every hostile page is drawn");

        walk_read(seed, bodies, &GetArticles { highlight: false, query: "sweep" });

        case_count = case_count.strict_add(1);
    }

    assert_eq!(case_count, case_count_max, "every case ran");

    case_count
}

fn node_hostile(prng: &mut Prng) -> Value {
    if prng.chance(WIDE_PERCENT) {
        return node_wide(prng);
    }

    match prng.index(14) {
        0 => json!(null),
        1 => json!(7_i64),
        2 => json!("a bare string where a node belongs"),
        3 => json!({"type": "text"}),
        4 => json!({"type": "text", "text": 7_i64}),
        5 => json!({"type": "paragraph", "content": 5_i64}),
        6 => json!({"type": "mention", "attrs": {"text": word_build(prng)}}),
        7 => json!({"type": "text", "text": word_build(prng), "marks": 5_i64}),
        8 => {
            let marks = json!([{"type": "link", "attrs": {"href": 7_i64}}]);

            json!({"type": "text", "text": "x", "marks": marks})
        }
        9 => json!({"type": "date", "attrs": {"timestamp": word_build(prng)}}),
        10 => json!({"type": "date", "attrs": {"timestamp": prng.int().to_string()}}),
        11 => {
            let millis = prng.index(1 << 54).to_string();

            json!({"type": "date", "attrs": {"timestamp": format!("-{millis}")}})
        }
        12 => json!({"type": "expand", "attrs": {"title": word_build(prng)}, "content": 5_i64}),
        _ => json!({"type": "hardBreak"}),
    }
}

fn node_wide(prng: &mut Prng) -> Value {
    let wide = "w".repeat(size::offset(WIDE_BYTES));

    if !prng.boolean() {
        return json!({"type": "text", "text": wide});
    }

    if prng.boolean() {
        return json!({"type": "nestedExpand", "attrs": {"title": wide}});
    }

    let href = word_build(prng);

    json!({
        "type": "text",
        "text": wide,
        "marks": [{"type": "link", "attrs": {"href": href}}],
    })
}

fn page_build(prng: &mut Prng) -> Value {
    if prng.chance(WIDE_PERCENT) {
        return page_member(prng);
    }

    let field_count = prng.index(PAGE_FIELD_COUNT_MAX).strict_add(1);
    let mut page = Map::new();

    for _field_index in 0..field_count {
        let field = pick(prng, &PAGE_FIELDS);

        let _replaced = page.insert(field.to_owned(), page_member(prng));
    }

    assert!(!page.is_empty(), "a page holds at least one member");

    Value::Object(page)
}

fn page_check(
    parsed: Result<Page<Text>, Error>,
    expected: Option<bool>,
    (seed, reply): (Seed, &Value),
) {
    assert!(seed < u64::MAX, "a seed is a number a run can report");

    let page = match parsed {
        Ok(page) => page,
        Err(failure) => {
            assert!(matches!(failure, Error::Reply(_)), "seed {seed}: a fault of the reply");
            assert!(expected.is_none(), "seed {seed}: {reply} was refused: {failure:?}");

            return;
        }
    };

    assert_eq!(Some(page.is_last()), expected, "seed {seed}: the wrong end of a walk: {reply}");

    assert!(
        size::of_offset(page.items().count()) <= page.count(),
        "seed {seed}: a walk hands back at most what it counted",
    );

    assert!(page.is_last() || !page.is_short(), "seed {seed}: only a last page can fall short");

    let count = page.count();

    assert_eq!(size::of_slice(&page.into_values()), count, "seed {seed}: no item is lost");
}

fn page_flag(reply: &Value) -> Option<bool> {
    assert!(!PAGE_FLAGS.is_empty(), "the end of a page has a spelling");

    PAGE_FLAGS.iter().find_map(|flag| reply.get(flag).and_then(Value::as_bool))
}

fn page_member(prng: &mut Prng) -> Value {
    match prng.index(9) {
        0 => json!(null),
        1 => json!(true),
        2 => json!(false),
        3 => json!(-1_i64),
        4 => json!(prng.index(1_000)),
        5 => json!(""),
        6 => json!(word_build(prng)),
        7 => json!([word_build(prng), 7_i64, null]),
        _ => json!([]),
    }
}

fn page_model(reply: &Value, paging: Paging, reached: u64) -> Option<bool> {
    assert!(reached <= limits::CURSOR_MAX, "a fixture stays inside the walk budget");

    let found = paging.items.map_or(Some(reply), |pointer| reply.pointer(pointer));
    let items = found?.as_array()?;
    let is_closed = items.is_empty() && page_flag(reply) == Some(true);

    if is_closed {
        return Some(true);
    }

    match paging.next {
        Next::Link(pointer) => page_model_next(reply, pointer, link_escaped(reply, pointer)),
        Next::Number | Next::Offset => page_model_position(reply, items, reached),
        Next::Token(pointer) => page_model_next(reply, pointer, token_held(reply, pointer)),
    }
}

fn page_model_next(reply: &Value, pointer: &str, usable: Option<(String, u32)>) -> Option<bool> {
    assert!(pointer.starts_with('/'), "a next token is named by a pointer");
    assert!(usable.as_ref().is_none_or(|(text, _bound)| !text.is_empty()), "a next says something");

    if let Some((text, bound)) = usable {
        return (size::of_text(&text) <= u64::from(bound)).then_some(false);
    }

    let held = reply.pointer(pointer).filter(|value| !value.is_null());
    let is_named = held.is_some_and(|value| value.as_str() != Some(""));

    match page_flag(reply) {
        Some(true) => Some(true),
        Some(false) => None,
        None => (!is_named).then_some(true),
    }
}

fn page_model_position(reply: &Value, items: &[Value], reached: u64) -> Option<bool> {
    assert!(reached <= limits::CURSOR_MAX, "a fixture stays inside the walk budget");
    assert!(size::of_slice(items) < u64::MAX, "a page has a countable size");

    if items.is_empty() {
        return (page_flag(reply) != Some(false)).then_some(true);
    }

    if let Some(flag) = page_flag(reply) {
        return Some(flag);
    }

    let total = reply.get("total").and_then(Value::as_u64);

    Some(total == Some(reached.strict_add(size::of_slice(items))))
}

#[must_use]
pub fn page_parse(seed: u64, case_count_max: u32) -> u32 {
    assert!(case_count_max >= 1, "seed {seed}: a run checks at least one case");

    let mut prng = Prng::from_seed(seed);
    let mut case_count: u32 = 0;

    for _case_index in 0..case_count_max {
        let reply = page_build(&mut prng);
        let paging = pick_paging(&mut prng);
        let cursor = prng.index(1_000);
        let expected = page_model(&reply, paging, cursor);
        let parsed = Page::<Text>::parse(reply.clone(), paging, cursor);

        page_check(parsed, expected, (seed, &reply));

        case_count = case_count.strict_add(1);
    }

    assert_eq!(case_count, case_count_max, "every case ran");

    case_count
}

fn pick<Item>(prng: &mut Prng, items: &[Item]) -> Item
where
    Item: Copy + Default,
{
    assert!(!items.is_empty(), "a pick draws from at least one item");

    let count = size::of_slice(items);
    let index = prng.index(count);

    assert!(index < count, "a pick stays inside its items");

    items.get(size::offset(index)).copied().unwrap_or_default()
}

fn pick_paging(prng: &mut Prng) -> Paging {
    let count = size::of_slice(&PAGINGS);
    let index = prng.index(count);

    assert!(index < count, "a pick stays inside its items");

    let fallback = Paging { cursor: "startAt", items: None, next: Next::Offset };

    PAGINGS.get(size::offset(index)).copied().unwrap_or(fallback)
}

fn reply_answer(body: &[u8], status: u16) -> Response {
    assert!(status >= 200, "only a success reads as a reply");

    Response {
        body: body.to_vec(),
        is_near_limit: false,
        location: Some("https://x/task/1".to_owned()),
        rate_limit_reason: None,
        retry_after_seconds: None,
        status,
        streamed_count: 0,
    }
}

fn reply_body_build(prng: &mut Prng) -> Vec<u8> {
    let piece_count = prng.index(REPLY_PIECE_COUNT_MAX);
    let mut body = Vec::new();

    for _piece_index in 0..piece_count {
        body.extend_from_slice(pick(prng, &REPLY_PIECES));
    }

    assert!(piece_count < REPLY_PIECE_COUNT_MAX, "a body holds at most its piece budget");

    body
}

fn reply_check(seed: Seed, body: &[u8], status: u16, request: &Request) {
    assert!(status >= 200, "seed {seed}: only a success reads as a reply");

    let held: Option<Value> = serde_json::from_slice(body).ok();
    let nothing = Reply::<Nothing>::parse(reply_answer(body, status), request);
    let text = Reply::<Text>::parse(reply_answer(body, status), request);
    let any = Reply::<Any>::parse(reply_answer(body, status), request);

    let nothing_keeps = nothing.as_ref().is_ok_and(|reply| {
        reply.read().is_some()
            && reply.value().is_null() == (body.is_empty() || held == Some(Value::Null))
    });

    let text_reads = text.is_ok_and(|reply| body.is_empty() || reply.read().is_some());

    assert!(nothing_keeps, "seed {seed}: {body:?} read as nothing: {nothing:?}");
    assert!(text_reads, "seed {seed}: {body:?} read as no text");

    let Ok(reply) = any else {
        let is_fault = matches!(any, Err(Error::Reply(_)));

        assert!(is_fault, "seed {seed}: {body:?} read as {any:?}");
        assert!(held.is_none(), "seed {seed}: readable {body:?} was refused");
        assert!(request.is_read(), "seed {seed}: a landed write failed on {body:?}");

        return;
    };

    assert_eq!(reply.status(), status, "seed {seed}: the status survives");
    assert_eq!(reply.location(), Some("https://x/task/1"), "seed {seed}: the location survives");

    if let Some(value) = held {
        assert_eq!(reply.value(), &value, "seed {seed}: {body:?} is the JSON it was");
    } else {
        let is_kept = body.is_empty() || reply.value().is_string();

        assert!(is_kept, "seed {seed}: {body:?} read as {:?}", reply.value());
    }
}

fn reply_deep_check(seed: Seed, prng: &mut Prng, request: &Request) {
    let depth = size::offset(prng.index(REPLY_DEPTH_MAX)).strict_add(1);
    let opener = if prng.boolean() { "[" } else { "{\"a\":" };
    let closer = if opener == "[" { "]" } else { "}" };
    let body = [opener.repeat(depth), "1".to_owned(), closer.repeat(depth)].concat();
    let parsed = Reply::<Any>::parse(reply_answer(body.as_bytes(), 200), request);

    assert!(request.is_read(), "seed {seed}: only a read can refuse a body");
    assert!(parsed.is_ok(), "seed {seed}: {depth} levels of {opener} read as {parsed:?}");
}

fn reply_float_check(seed: Seed, prng: &mut Prng, request: &Request) {
    let number = f64::from_bits(prng.int());

    if !number.is_finite() {
        return;
    }

    let text = format!("{number:e}");
    let parsed = Reply::<Any>::parse(reply_answer(text.as_bytes(), 200), request);
    let held = parsed.ok().and_then(|reply| reply.value().as_f64());

    assert_eq!(held.map(f64::to_bits), Some(number.to_bits()), "seed {seed}: {text} moved");
}

fn reply_is_unreadable(body: &[u8], attempted: Attempted) -> bool {
    if attempted == Attempted::Write {
        return false;
    }

    if body.is_empty() {
        return false;
    }

    serde_json::from_slice::<Value>(body).is_err()
}

#[must_use]
pub fn reply_parse(seed: u64, case_count_max: u32) -> u32 {
    assert!(case_count_max >= 1, "seed {seed}: a run checks at least one case");

    let mut prng = Prng::from_seed(seed);
    let mut case_count: u32 = 0;
    let read = Request::of::<GetServerInfo>(URL_SITE.to_owned(), "Basic fuzz");
    let write = Request::of::<CreateIssue<'_>>(URL_SITE.to_owned(), "Basic fuzz");

    for _case_index in 0..case_count_max {
        let body = reply_body_build(&mut prng);
        let status = pick(&mut prng, &REPLY_STATUSES);

        reply_check(seed, &body, status, &read);
        reply_check(seed, &body, status, &write);
        reply_deep_check(seed, &mut prng, &read);
        reply_float_check(seed, &mut prng, &read);

        case_count = case_count.strict_add(1);
    }

    assert_eq!(case_count, case_count_max, "every case ran");

    case_count
}

#[must_use]
pub fn request_url(seed: u64, case_count_max: u32) -> u32 {
    assert!(case_count_max >= 1, "seed {seed}: a run checks at least one case");

    let mut prng = Prng::from_seed(seed);
    let mut case_count: u32 = 0;

    for _case_index in 0..case_count_max {
        let id = url_value_build(&mut prng);
        let key = url_value_build(&mut prng);
        let expand = url_value_build(&mut prng);
        let operation = GetComment { id: &id, issue_id_or_key: &key };
        let options = get_comment::Options { expand: Some(&expand) };
        let url = format!("{URL_BASE}{}", <GetComment<'_> as Operation>::PATH);
        let mut request = Request::of::<GetComment<'_>>(url, "Basic secret");
        let filled = operation.request(&options, &mut request);

        if let Err(failure) = filled {
            assert!(matches!(failure, Error::Input(_)), "seed {seed}: a caller's fault");
        } else {
            request_url_check(&request, (&key, &id, &expand), seed);
        }

        case_count = case_count.strict_add(1);
    }

    assert_eq!(case_count, case_count_max, "every case ran");

    case_count
}

fn request_url_check(request: &Request, (key, id, expand): (&str, &str, &str), seed: u64) {
    assert!(!key.is_empty(), "seed {seed}: a filled key is never empty");

    let url = request.url();
    let tail = url.strip_prefix(URL_BASE).unwrap_or_default();

    assert!(tail.contains('?'), "seed {seed}: a set option always reaches the query");

    let (path, query) = tail.split_once('?').unwrap_or_default();
    let segments: Vec<&str> = path.split('/').collect();

    assert!(request.is_complete(), "seed {seed}: no placeholder survives in {url}");
    assert_eq!(segments.len(), 8, "seed {seed}: a hostile value added a path segment to {url}");
    assert_eq!(query.matches('&').count(), 0, "seed {seed}: a hostile value added a query term");
    assert_eq!(query.matches('?').count(), 0, "seed {seed}: a hostile value added a query mark");

    let decoded_key = percent_decode(segments.get(5).copied().unwrap_or_default());
    let decoded_id = percent_decode(segments.get(7).copied().unwrap_or_default());
    let decoded_expand = percent_decode(query.trim_start_matches("expand="));

    assert_eq!(decoded_key, key.as_bytes(), "seed {seed}: the key round trips");
    assert_eq!(decoded_id, id.as_bytes(), "seed {seed}: the id round trips");
    assert_eq!(decoded_expand, expand.as_bytes(), "seed {seed}: the option round trips");
}

fn retry_check(seed: Seed, faults: &[Fault], attempted: Attempted, network: &Faulty) {
    assert!(!faults.is_empty(), "seed {seed}: a script answers at least once");

    let attempt_count = network.answered_count.lock().map_or(0, |count| *count);
    let attempt_expected = attempt_count_expected(faults, attempted);
    let waits = network.waits.lock().map_or_else(|_poisoned| Vec::new(), |found| found.clone());
    let is_expected = attempt_count == attempt_expected;

    assert!(is_expected, "seed {seed}: {faults:?} on a {attempted:?} ran {attempt_count} attempts");

    let retry_count = attempt_expected.strict_sub(1);

    assert!(
        size::of_slice(&waits) >= retry_count,
        "seed {seed}: {retry_count} retries took fewer waits than that",
    );

    for (fault, wait) in faults.iter().zip(&waits).take(size::offset(retry_count)) {
        let floor = match fault {
            Fault::Answer {
                body: _,
                is_near_limit: _,
                retry_after_seconds: Some(seconds),
                status: _,
            } => *seconds,
            Fault::Answer { body: _, is_near_limit: _, retry_after_seconds: None, status: _ }
            | Fault::Dropped => 0,
        };

        assert!(wait.as_secs() >= floor, "seed {seed}: a wait of {wait:?} undercut {floor}");

        assert!(
            wait.as_millis() <= u128::from(limits::WAIT_MILLIS_MAX),
            "seed {seed}: a wait of {wait:?} ran past its cap",
        );
    }
}

fn retry_run(seed: Seed, faults: &[Fault], attempted: Attempted) {
    assert!(!faults.is_empty(), "seed {seed}: a script answers at least once");

    let network = Faulty {
        answered_count: Mutex::new(0),
        faults: faults.to_vec(),
        waits: Mutex::new(Vec::new()),
    };

    let built = Client::new(network, &CREDENTIALS);

    assert!(built.is_ok(), "seed {seed}: the fixture credentials are sound");

    let Ok(client) = built else {
        return;
    };

    let outcome = match attempted {
        Attempted::Read => client.send(&GetServerInfo).map(|_reply| ()),
        Attempted::Write => {
            let body = json!({"fields": {}});

            client.send(&CreateIssue { body: &body }).map(|_reply| ())
        }
    };

    let attempt_expected = attempt_count_expected(faults, attempted);
    let last = faults.get(size::offset(attempt_expected.strict_sub(1))).copied();

    retry_check(seed, faults, attempted, client.network());

    assert!(last.is_some(), "seed {seed}: a script answers every attempt");

    let Some(fault) = last else {
        return;
    };

    fault_outcome_check(seed, fault, attempted, &outcome);
}

#[must_use]
pub fn run(name: &str, seed: u64, case_count: u32) -> Option<u32> {
    assert!(case_count >= 1, "fuzzer {name} at seed {seed}: a run checks at least one case");

    let ran = match name {
        "adf" => Some(adf_flatten(seed, case_count)),
        "client" => Some(client_retry(seed, case_count)),
        "link" => Some(link_walk(seed, case_count)),
        "page" => Some(page_parse(seed, case_count)),
        "reply" => Some(reply_parse(seed, case_count)),
        "text" => Some(text_prefix(seed, case_count)),
        "url" => Some(request_url(seed, case_count)),
        "walk" => Some(walk_model(seed, case_count)),
        _ => None,
    };

    assert_eq!(ran.is_some(), NAMES.contains(&name), "the registry names exactly what runs");
    assert!(ran.is_none_or(|count| count == case_count), "a fuzzer runs every case it was asked");

    ran
}

fn status_is_success(status: u16, attempted: Attempted) -> bool {
    assert!(status >= 100, "a scripted answer carries a real status");

    if (200..=299).contains(&status) {
        return true;
    }

    if status != 303 {
        return false;
    }

    attempted == Attempted::Write
}

fn status_may_have_landed(status: u16, attempted: Attempted) -> bool {
    assert!(status >= 100, "a scripted answer carries a real status");

    if (400..500).contains(&status) {
        return false;
    }

    matches!(attempted, Attempted::Write)
}

fn term_of(request: &Request, name: &str) -> Option<u64> {
    assert!(!name.is_empty(), "a query term is named");

    request.query_of(name)?.parse().ok()
}

#[must_use]
pub fn text_prefix(seed: u64, case_count_max: u32) -> u32 {
    assert!(case_count_max >= 1, "seed {seed}: a run checks at least one case");

    let mut prng = Prng::from_seed(seed);
    let mut case_count: u32 = 0;

    for _case_index in 0..case_count_max {
        let raw = word_build(&mut prng);
        let raw_bytes = size::of_text(&raw);
        let floor_limit = prng.index(raw_bytes.strict_add(1));
        let boundary = text::boundary_floor(&raw, floor_limit);

        assert!(raw.is_char_boundary(size::offset(boundary)), "seed {seed}: a whole character");
        assert!(boundary <= floor_limit, "seed {seed}: the boundary stays under its limit");

        let cut_limit = text::CODE_POINT_BYTES_MAX.strict_add(prng.index(raw_bytes.strict_add(1)));
        let cut = text::prefix(&raw, cut_limit);

        assert!(size::of_text(cut) <= cut_limit, "seed {seed}: the cut stays under its limit");
        assert!(raw.starts_with(cut), "seed {seed}: the cut is a prefix of the text");

        case_count = case_count.strict_add(1);
    }

    assert_eq!(case_count, case_count_max, "every case ran");

    case_count
}

fn token_held(reply: &Value, pointer: &str) -> Option<(String, u32)> {
    assert!(pointer.starts_with('/'), "a next token is named by a pointer");

    let text = reply.pointer(pointer)?.as_str()?;

    (!text.is_empty()).then(|| (text.to_owned(), limits::TOKEN_BYTES_MAX))
}

fn tree_fold(prng: &mut Prng, leaves: Vec<Value>, types: &[&str]) -> Vec<Value> {
    assert!(!leaves.is_empty(), "a fold starts from at least one leaf");
    assert!(!types.is_empty(), "a fold has a node type to fold into");

    let mut level = leaves;

    for _depth_index in 0..DEPTH_MAX {
        if level.len() <= 1 {
            break;
        }

        let level_count = size::of_slice(&level);
        let mut next: Vec<Value> = Vec::with_capacity(level.len());
        let mut taken: u64 = 0;

        for _group_index in 0..level_count {
            if taken >= level_count {
                break;
            }

            let room = level_count.strict_sub(taken);
            let group = prng.index(GROUP_COUNT_MAX).strict_add(1).min(room);

            let children: Vec<Value> =
                level.iter().skip(size::offset(taken)).take(size::offset(group)).cloned().collect();

            let kind = pick(prng, types);

            next.push(json!({"type": kind, "content": children}));

            taken = taken.strict_add(group);
        }

        assert_eq!(taken, level_count, "a fold takes every node once");

        level = next;
    }

    assert!(!level.is_empty(), "a fold keeps at least one node");

    level
}

fn url_value_build(prng: &mut Prng) -> String {
    let piece_count = prng.index(URL_PIECE_COUNT_MAX).strict_add(1);
    let mut value = String::new();

    for _piece_index in 0..piece_count {
        value.push_str(pick(prng, &URL_PIECES));
    }

    assert!(!value.is_empty(), "a value holds at least one piece");
    assert!(piece_count <= URL_PIECE_COUNT_MAX, "a value holds at most its piece budget");

    value
}

fn walk_check<Walked>(seed: Seed, model: Model, operation: &Walked)
where
    Walked: Paged,
{
    assert!(model.page_item_count >= 1, "seed {seed}: a page holds at least one item");

    let network = Modeled { model, request_count: Mutex::new(0) };
    let built = Client::new(network, &CREDENTIALS);

    assert!(built.is_ok(), "seed {seed}: the fixture credentials are sound");

    let Ok(client) = built else {
        return;
    };

    let mut values: Vec<Value> = Vec::new();

    for page in client.walk(operation).take(size::offset(WALK_ITEM_COUNT_MAX.strict_add(2))) {
        let failure = page.as_ref().err();

        assert!(failure.is_none(), "seed {seed}: {model:?} failed: {failure:?}");

        values.extend(page.map(Page::into_values).unwrap_or_default());
    }

    let expected: Vec<Value> = (0..model.item_count).map(|item| json!(item)).collect();
    let request_count = client.network().request_count.lock().map_or(0, |count| *count);

    assert_eq!(values, expected, "seed {seed}: {model:?} lost or repeated an item");
    assert_eq!(request_count, model.request_count_expected(), "seed {seed}: {model:?}");
}

#[must_use]
pub fn walk_model(seed: u64, case_count_max: u32) -> u32 {
    assert!(case_count_max >= 1, "seed {seed}: a run checks at least one case");

    let mut prng = Prng::from_seed(seed);
    let mut case_count: u32 = 0;
    let articles = GetArticles { highlight: false, query: "sweep" };
    let objects = AQLFindObjects { workspace_id: "fuzz" };

    for _case_index in 0..case_count_max {
        let item_count = prng.index(WALK_ITEM_COUNT_MAX.strict_add(1));
        let page_item_count = prng.index(WALK_PAGE_ITEM_COUNT_MAX).strict_add(1);
        let bare = Model { item_count, page_item_count, style: Style::Bare };

        walk_check(seed, bare, &GetAllUsersDefault);
        walk_check(seed, bare.styled(Style::Flag), &SearchProjects);
        walk_check(seed, bare.styled(Style::Link), &articles);
        walk_check(seed, bare.styled(Style::Number), &objects);
        walk_check(seed, bare.styled(Style::Since), &GetIDsOfWorklogsModifiedSince);
        walk_check(seed, bare.styled(Style::Token), &SearchAndReconsileIssuesUsingJQL);
        walk_check(seed, bare.styled(Style::Total), &SearchProjects);

        case_count = case_count.strict_add(1);
    }

    assert_eq!(case_count, case_count_max, "every case ran");

    case_count
}

fn walk_read<Walked>(seed: Seed, bodies: Vec<Value>, operation: &Walked)
where
    Walked: Paged,
{
    assert!(!bodies.is_empty(), "seed {seed}: a walk reads at least one page");

    let page_count_max = size::of_slice(&bodies).strict_add(1);
    let network = Hostile { bodies: Mutex::new(bodies) };
    let built = Client::new(network, &CREDENTIALS);

    assert!(built.is_ok(), "seed {seed}: the fixture credentials are sound");

    let Ok(client) = built else {
        return;
    };

    let mut walk = client.walk(operation);

    for _page_index in 0..page_count_max {
        let answer = walk.next();

        assert!(answer.is_some(), "seed {seed}: a page that is not last leads on");

        let Some(Ok(page)) = answer else {
            let after = walk.next();

            assert!(after.is_none(), "seed {seed}: a failed walk stays failed");

            return;
        };

        if page.is_last() {
            let after = walk.next();

            assert!(after.is_none(), "seed {seed}: a last page ends the walk");

            return;
        }
    }

    let following = walk.next();

    assert!(following.is_none_or(|page| page.is_err()), "seed {seed}: a walk ran past its script");
}

fn walk_run(seed: Seed, prng: &mut Prng) {
    let body_count = prng.index(PAGE_HOSTILE_COUNT_MAX).strict_add(1);

    let bodies: Vec<Value> =
        iter::repeat_with(|| page_build(prng)).take(size::offset(body_count)).collect();

    assert_eq!(size::of_slice(&bodies), body_count, "every hostile page is drawn");

    walk_read(seed, bodies.clone(), &SearchProjects);
    walk_read(seed, bodies, &SearchAndReconsileIssuesUsingJQL);
}

fn word_build(prng: &mut Prng) -> String {
    if prng.chance(WIDE_PERCENT) {
        return "w".repeat(size::offset(WIDE_BYTES));
    }

    let character_count = prng.index(WORD_CHARACTER_COUNT_MAX).strict_add(1);
    let mut word = String::with_capacity(size::offset(character_count));

    for _character_index in 0..character_count {
        word.push(pick(prng, &WORD_ALPHABET));
    }

    assert!(!word.is_empty(), "a word holds at least one character");

    assert_eq!(
        size::of_offset(word.chars().count()),
        character_count,
        "a word holds the characters drawn for it",
    );

    word
}

#[cfg(test)]
mod tests {
    use super::{
        CASE_COUNT_SMOKE,
        SEED_SMOKE,
        adf_flatten,
        client_retry,
        document_hostile,
        document_planted,
        link_walk,
        page_parse,
        reply_parse,
        request_url,
        text_prefix,
        walk_model,
        word_build,
    };
    use crate::prng::Prng;

    #[test]
    fn a_seed_replays_the_same_inputs_and_another_seed_draws_new_ones() {
        let mut first = Prng::from_seed(7);
        let mut second = Prng::from_seed(7);
        let mut other = Prng::from_seed(8);
        let mut is_identical = true;

        for _case_index in 0..32_u32 {
            let word = word_build(&mut first);
            let planted = document_planted(&mut first);
            let hostile = document_hostile(&mut first);

            assert_eq!(word, word_build(&mut second), "one seed draws one word sequence");
            assert_eq!(planted, document_planted(&mut second), "one seed plants one tree");
            assert_eq!(hostile, document_hostile(&mut second), "one seed draws one hostile tree");

            is_identical &= word == word_build(&mut other);
        }

        assert!(!is_identical, "another seed draws another sequence");
    }

    #[test]
    fn the_adf_flattener_survives_a_smoke_run() {
        assert_eq!(adf_flatten(SEED_SMOKE, CASE_COUNT_SMOKE), CASE_COUNT_SMOKE, "all ran");
    }

    #[test]
    fn the_client_keeps_its_retry_policy_through_a_smoke_run() {
        assert_eq!(client_retry(SEED_SMOKE, CASE_COUNT_SMOKE), CASE_COUNT_SMOKE, "all ran");
    }

    #[test]
    fn the_link_walk_stays_on_its_site_through_a_smoke_run() {
        assert_eq!(link_walk(SEED_SMOKE, CASE_COUNT_SMOKE), CASE_COUNT_SMOKE, "all ran");
    }

    #[test]
    fn the_page_parser_survives_a_smoke_run() {
        assert_eq!(page_parse(SEED_SMOKE, CASE_COUNT_SMOKE), CASE_COUNT_SMOKE, "all ran");
    }

    #[test]
    fn the_reply_reader_survives_a_smoke_run() {
        assert_eq!(reply_parse(SEED_SMOKE, CASE_COUNT_SMOKE), CASE_COUNT_SMOKE, "all ran");
    }

    #[test]
    fn the_request_builder_survives_a_smoke_run() {
        assert_eq!(request_url(SEED_SMOKE, CASE_COUNT_SMOKE), CASE_COUNT_SMOKE, "all ran");
    }

    #[test]
    fn the_text_cutter_survives_a_smoke_run() {
        assert_eq!(text_prefix(SEED_SMOKE, CASE_COUNT_SMOKE), CASE_COUNT_SMOKE, "all ran");
    }

    #[test]
    fn the_walk_of_every_paging_style_keeps_every_item_through_a_smoke_run() {
        assert_eq!(walk_model(SEED_SMOKE, CASE_COUNT_SMOKE), CASE_COUNT_SMOKE, "all ran");
    }
}
