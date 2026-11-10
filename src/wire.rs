use alloc::collections::VecDeque;
use alloc::sync::Arc;
use core::net::SocketAddr;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use core::time::Duration;
use std::env;
use std::io::{self, Cursor, Read as _, Seek, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::thread::{self, JoinHandle};

use serde_json::json;
use ureq::config::Config;
use ureq::http::Uri;
use ureq::unversioned::resolver::{ResolvedSocketAddrs, Resolver};
use ureq::unversioned::transport::{
    Buffers,
    ConnectionDetails,
    Connector,
    LazyBuffers,
    NextTimeout,
    TcpConnector,
    Transport as Carrier,
};

use crate::agent::Agent;
use crate::client::Client;
use crate::credentials::Credentials;
use crate::error::Error;
use crate::limits;
use crate::part::Part;
use crate::platform::{
    AddAttachment,
    ArchiveProject,
    CreateIssue,
    DeleteRemoteIssueLinkByGlobalID,
    GetAttachmentContent,
    GetServerInfo,
    GetValidProjectKey,
    SearchProjects,
    get_valid_project_key,
    search_projects,
};
use crate::prng::Prng;
use crate::request::Request;
use crate::response::Response;
use crate::size;
use crate::transport::Transport;
use crate::wait::Wait;

const AUTHORIZATION: &str = "Basic c2VjcmV0";
const BYTES_MAX: u64 = 1024 * 1024;

const GZIP_HEAD: &[u8] =
    b"\x1f\x8b\x08\0\0\0\0\0\x02\xff\xed\xc1\x01\x01\0\0\0\x80\x90\xfe\xaf\xee\x08\x0a";

const GZIP_TAIL: &[u8] = b"\x6a\xeb\x8e\x97\xd7\0\0\x01\0";
const GZIP_UNPACKED_BYTES: u64 = 64 * 1024;
const GZIP_ZERO_RUN_BYTES: usize = 63;
const HOSTILE_BUFFER_BYTES: u64 = 16 * 1024;
const HOSTILE_CASES_DEFAULT: u64 = 8_000;
const HOSTILE_CONNECTION_COUNT_MAX: u64 = 3;

const HOSTILE_BODIES: [&[u8]; 16] = [
    b"",
    b"hello",
    b"{}",
    b"null",
    b"0\r\n",
    b"0\r\n\r\n",
    b"5\r\nhello\r\n0\r\n\r\n",
    b"5\r\nhe",
    b"zz\r\n",
    b"ffffffffffffffff\r\n",
    b"0;a\r\nTrailer: x\r\n",
    b"\x1f\x8b\x08\0\0\0\0\0\x02\xff",
    b"\x1f\x8b\x08\0\0\0\0\0\x02\xff\x03\0\0\0\0\0\0\0\0\0",
    b"{\"values\":[1],\"isLast\":false}",
    b"{\"values\":[],\"isLast\":true}",
    b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n",
];

const HOSTILE_CONNECTIONS: [&str; 4] = ["close", "keep-alive", "upgrade", "close, keep-alive"];
const HOSTILE_ENCODINGS: [&str; 6] = ["gzip", "br", "deflate", "identity", "GZIP", "gzip, gzip"];

const HOSTILE_LENGTHS: [&str; 9] =
    ["0", "1", "5", "100", "-1", "abc", "5, 6", "99999999999999999999", ""];

const HOSTILE_LOCATIONS: [&str; 32] = [
    "/next",
    "HTTPS://api.atlassian.com/../evil.example/x",
    "HTTPS://evil.example/../api.atlassian.com/x",
    "HTTPS://api.atlassian.com/rest/api/3/moved",
    "hTTpS://api.media.atlassian.com/file/1",
    "HTTP://plain/x",
    "#frag",
    "../up/x",
    ".",
    "./x",
    "?a=1",
    "https://api.media.atlassian.com/file/1",
    "",
    "//",
    "?",
    "#",
    "http://[",
    "https://",
    "https:///x",
    "http://plain/x",
    ":",
    "a:b",
    "\\\\x",
    "/../..",
    "https://user:pw@x/y",
    "ftp://x/y",
    "https://x:99999/y",
    "https://x/\u{e9}",
    "/%zz",
    "..",
    "/a b",
    "https:",
];

const HOSTILE_PRELUDES: [&str; 6] = [
    "",
    "",
    "HTTP/1.1 100 Continue\r\n\r\n",
    "HTTP/1.1 103 Early\r\nLink: x\r\n\r\n",
    "HTTP/1.1 199 x\r\nTransfer-Encoding: chunked\r\n\r\n",
    "\r\n",
];

const HOSTILE_RETRIES: [&str; 6] = ["0", "1", "900", "901", "soon", "18446744073709551616"];

const HOSTILE_STATUSES: [&str; 26] = [
    "100",
    "101",
    "199",
    "200",
    "200",
    "201",
    "202",
    "204",
    "205",
    "206",
    "299",
    "300",
    "301",
    "302",
    "303",
    "304",
    "307",
    "308",
    "399",
    "400",
    "404",
    "429",
    "500",
    "503",
    "600",
    "999",
];

const HOSTILE_TRANSFERS: [&str; 5] =
    ["chunked", "gzip", "gzip, chunked", "chunked, gzip", "identity"];

const HOSTILE_VERSIONS: [&str; 5] = ["HTTP/1.1", "HTTP/1.1", "HTTP/1.0", "HTTP/1.2", "HTTP/2.0"];
const POLL_COUNT_MAX: u32 = 15_000;
const POLL_MILLIS: u64 = 2;
const READ_COUNT_MAX: u32 = 1_024;
const READ_TIMEOUT_SECONDS: u64 = 30;
const REDIRECT_COUNT_MAX: u32 = 8;
const URL_READ: &str = "https://api.atlassian.com/ex/jira/site/rest/api/3/serverInfo";

const CREDENTIALS: Credentials<'static> =
    Credentials::Token { cloud_id: "site", email: "user@example.com", token: "token123" };

#[derive(Debug)]
struct Disguised {
    carrier: Box<dyn Carrier>,
}

#[derive(Debug)]
struct Instant {
    agent: Agent,
    waits: Mutex<Vec<Duration>>,
}

#[derive(Debug)]
struct Memory {
    buffers: LazyBuffers,
    pieces: VecDeque<Vec<u8>>,
    writes_sent: Arc<AtomicU64>,
}

#[derive(Debug)]
struct MemoryConnector {
    connections: Mutex<VecDeque<VecDeque<Vec<u8>>>>,
    writes_sent: Arc<AtomicU64>,
}

#[derive(Debug)]
struct Local {
    address: SocketAddr,
}

#[derive(Debug)]
struct Loopback;

#[derive(Clone, Debug)]
enum Scene {
    Hang,
    Say(Vec<u8>),
    Stall(Vec<u8>),
    Vanish,
}

#[derive(Debug)]
struct Server {
    address: SocketAddr,
    handle: JoinHandle<Vec<String>>,
    is_finished: Arc<AtomicBool>,
}

#[derive(Debug)]
struct SinkRefusing;

impl Carrier for Disguised {
    fn await_input(&mut self, timeout: NextTimeout) -> Result<bool, ureq::Error> {
        self.carrier.await_input(timeout)
    }

    fn buffers(&mut self) -> &mut dyn Buffers {
        self.carrier.buffers()
    }

    fn is_open(&mut self) -> bool {
        self.carrier.is_open()
    }

    fn is_tls(&self) -> bool {
        assert!(format!("{self:?}").contains("carrier"), "a disguise wraps a carrier");

        true
    }

    fn transmit_output(&mut self, amount: usize, timeout: NextTimeout) -> Result<(), ureq::Error> {
        assert!(amount >= 1, "a transmit moves at least one byte");

        self.carrier.transmit_output(amount, timeout)
    }
}

impl Carrier for Memory {
    fn await_input(&mut self, _timeout: NextTimeout) -> Result<bool, ureq::Error> {
        let Some(mut piece) = self.pieces.pop_front() else {
            return Ok(false);
        };

        let room = self.buffers.input_append_buf();
        let count = piece.len().min(room.len());

        room.get_mut(..count)
            .unwrap_or_default()
            .copy_from_slice(piece.get(..count).unwrap_or_default());

        self.buffers.input_appended(count);

        if count < piece.len() {
            drop(piece.drain(..count));
            self.pieces.push_front(piece);
        }

        Ok(count > 0)
    }

    fn buffers(&mut self) -> &mut dyn Buffers {
        &mut self.buffers
    }

    fn is_open(&mut self) -> bool {
        !self.pieces.is_empty()
    }

    fn is_tls(&self) -> bool {
        assert!(!format!("{self:?}").is_empty(), "a memory link can be printed");

        true
    }

    fn transmit_output(&mut self, amount: usize, _timeout: NextTimeout) -> Result<(), ureq::Error> {
        assert!(amount >= 1, "a transmit moves at least one byte");

        if self.buffers.output().starts_with(b"POST ") {
            let _sent = self.writes_sent.fetch_add(1, Ordering::SeqCst);
        }

        Ok(())
    }
}

impl Connector for Loopback {
    type Out = Disguised;

    fn connect(
        &self,
        details: &ConnectionDetails<'_>,
        chained: Option<()>,
    ) -> Result<Option<Disguised>, ureq::Error> {
        let carrier = TcpConnector::default().connect(details, chained)?;

        Ok(carrier.map(|found| Disguised { carrier: found.boxed() }))
    }
}

impl Connector for MemoryConnector {
    type Out = Memory;

    fn connect(
        &self,
        details: &ConnectionDetails<'_>,
        _chained: Option<()>,
    ) -> Result<Option<Memory>, ureq::Error> {
        assert!(details.uri.scheme().is_some(), "a connection names its scheme");

        let next = self.connections.lock().map_or(None, |mut held| held.pop_front());

        let Some(pieces) = next else {
            return Err(ureq::Error::ConnectionFailed);
        };

        Ok(Some(Memory {
            buffers: LazyBuffers::new(
                size::offset(HOSTILE_BUFFER_BYTES),
                size::offset(HOSTILE_BUFFER_BYTES),
            ),
            pieces,
            writes_sent: Arc::clone(&self.writes_sent),
        }))
    }
}

impl Resolver for Local {
    fn resolve(
        &self,
        _uri: &Uri,
        _config: &Config,
        _timeout: NextTimeout,
    ) -> Result<ResolvedSocketAddrs, ureq::Error> {
        let mut addresses = self.empty();

        addresses.push(self.address);

        Ok(addresses)
    }
}

impl Server {
    fn finish(self) -> Vec<String> {
        self.is_finished.store(true, Ordering::SeqCst);

        self.handle.join().expect("the server ends")
    }

    fn start(scenes: Vec<Scene>) -> Self {
        assert!(!scenes.is_empty(), "a server plays at least one scene");

        let listener = TcpListener::bind("127.0.0.1:0").expect("a free port");
        let address = listener.local_addr().expect("a bound port");

        listener.set_nonblocking(true).expect("a polling listener");

        let is_finished = Arc::new(AtomicBool::new(false));
        let flag = Arc::clone(&is_finished);
        let handle = thread::spawn(move || scenes_play(&listener, scenes, &flag));

        Self { address, handle, is_finished }
    }
}

impl Transport for Instant {
    fn fetch(&self, request: &Request, bytes_max: u64) -> Result<Response, Error> {
        assert!(bytes_max >= 1, "a body may hold at least one byte");

        self.agent.fetch(request, bytes_max)
    }

    fn stream<Sink>(&self, request: &Request, sink: &mut Sink) -> Result<Response, Error>
    where
        Sink: Seek + Write,
    {
        self.agent.stream(request, sink)
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

impl Seek for SinkRefusing {
    fn seek(&mut self, position: io::SeekFrom) -> io::Result<u64> {
        assert!(
            matches!(position, io::SeekFrom::Start(0) | io::SeekFrom::Current(0)),
            "a sink only ever stays where it began",
        );

        Ok(0)
    }
}

impl Write for SinkRefusing {
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        assert!(!bytes.is_empty(), "a sink is asked to hold something");

        Err(io::Error::other("the disk is full"))
    }
}

#[test]
fn a_bodiless_write_names_json_and_a_read_names_nothing() {
    let server = Server::start(vec![answer(204, &[], ""), answer(200, &[], "{}")]);
    let client = client_of(&server);

    let archived = client.send(&ArchiveProject { project_id_or_key: "SWP" }).expect("a write");
    let server_read = client.send(&GetServerInfo).expect("a read");
    let seen = server.finish();
    let write = seen[0].to_lowercase();
    let read = seen[1].to_lowercase();

    assert_eq!(archived.status(), 204, "the write landed");
    assert!(server_read.read().is_some(), "an object answer reads");

    assert!(write.starts_with("post "), "the method survives: {write}");
    assert!(write.contains("content-type: application/json"), "a write says JSON: {write}");
    assert!(seen[0].ends_with("\r\n\r\n"), "no body follows the headers: {}", seen[0]);
    assert!(!read.contains("content-type"), "a read names no type: {read}");
}

#[test]
fn a_body_at_its_limit_lands_and_one_byte_more_is_a_fault_of_the_reply() {
    let server = Server::start(vec![answer(200, &[], "12345"), answer(200, &[], "12345")]);
    let agent = agent_of(&server);
    let landed = agent.fetch(&request_read(), 5).expect("a body at its limit");
    let failure = agent.fetch(&request_read(), 4).expect_err("a refusal");

    assert_eq!(landed.body, b"12345", "at the limit");
    assert!(matches!(failure, Error::Reply(_)), "a larger reply stays larger: {failure:?}");
    assert!(failure.message().contains("4 byte limit"), "the failure names the limit");
    assert_eq!(server.finish().len(), 2, "each answer was asked for once");
}

#[test]
fn a_body_cut_short_is_a_transport_failure() {
    let scene = said("HTTP/1.1 200 OK\r\nContent-Length: 100\r\nConnection: close\r\n\r\nshort");
    let server = Server::start(vec![scene]);
    let failure = agent_of(&server).fetch(&request_read(), BYTES_MAX).expect_err("a failure");

    assert!(matches!(failure, Error::Transport(_)), "the wire failed: {failure:?}");
    assert_eq!(server.finish().len(), 1, "one request was made");
}

#[test]
fn a_body_that_never_came_is_a_transport_failure() {
    let scene = said("HTTP/1.1 200 OK\r\nContent-Length: 100\r\nConnection: close\r\n\r\n");
    let server = Server::start(vec![scene]);
    let failure = agent_of(&server).fetch(&request_read(), BYTES_MAX).expect_err("a failure");

    assert!(matches!(failure, Error::Transport(_)), "nothing proves it was empty: {failure:?}");
    assert_eq!(server.finish().len(), 1, "one request was made");
}

#[test]
fn a_chain_of_redirects_that_leaves_its_origin_never_signs_again() {
    let away = answer(302, &[("Location", "https://api.media.atlassian.com/file")], "");
    let back = answer(302, &[("Location", "https://api.atlassian.com/rest/api/3/back")], "");
    let server = Server::start(vec![away, back, answer(200, &[], "landed")]);
    let response = agent_of(&server).fetch(&request_read(), BYTES_MAX).expect("an answer");
    let seen = server.finish();

    assert_eq!(response.body, b"landed", "the chain was followed");
    assert!(seen[0].to_lowercase().contains("authorization: basic"), "the first hop is signed");
    assert!(!seen[1].to_lowercase().contains("authorization"), "the second left the origin");
    assert!(!seen[2].to_lowercase().contains("authorization"), "and the way back stays unsigned");
}

#[test]
fn a_chunked_body_cut_short_is_a_transport_failure() {
    let scene = said(
        "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n5\r\nhello\r\n",
    );

    let server = Server::start(vec![scene]);
    let failure = agent_of(&server).fetch(&request_read(), BYTES_MAX).expect_err("a failure");

    assert!(matches!(failure, Error::Transport(_)), "the wire failed: {failure:?}");
    assert_eq!(server.finish().len(), 1, "one request was made");
}

#[test]
fn a_chunked_body_reads_whole() {
    let scene = said(
        "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n\
         5\r\nhello\r\n6\r\n world\r\n0\r\n\r\n",
    );

    let server = Server::start(vec![scene]);
    let response = agent_of(&server).fetch(&request_read(), BYTES_MAX).expect("an answer");

    assert_eq!(response.body, b"hello world", "each chunk arrives in order");
    assert_eq!(server.finish().len(), 1, "one request was made");
}

#[test]
fn a_compressed_body_cut_part_way_is_a_transport_failure() {
    let head = "HTTP/1.1 200 OK\r\nContent-Encoding: gzip\r\nConnection: close\r\n\r\n";
    let zeros = [0; GZIP_ZERO_RUN_BYTES.strict_div(2)];
    let scene = Scene::Say([head.as_bytes(), GZIP_HEAD, &zeros].concat());
    let server = Server::start(vec![scene]);
    let failure = agent_of(&server).fetch(&request_read(), BYTES_MAX).expect_err("a failure");

    assert!(matches!(failure, Error::Transport(_)), "only part of it unpacked: {failure:?}");
    assert_eq!(server.finish().len(), 1, "one request was made");
}

#[test]
fn a_compressed_body_is_measured_after_it_is_unpacked() {
    let server = Server::start(vec![gzip_scene(), gzip_scene()]);
    let agent = agent_of(&server);
    let landed = agent.fetch(&request_read(), GZIP_UNPACKED_BYTES).expect("a body at its limit");
    let failure = agent.fetch(&request_read(), GZIP_UNPACKED_BYTES.strict_sub(1)).expect_err("big");

    assert_eq!(size::of_slice(&landed.body), GZIP_UNPACKED_BYTES, "the whole of it");
    assert!(landed.body.iter().all(|byte| *byte == 0), "and unpacked");
    assert!(matches!(failure, Error::Reply(_)), "96 bytes on the wire: {failure:?}");
    assert_eq!(server.finish().len(), 2, "neither read was repeated");
}

#[test]
fn a_compressed_download_lands_unpacked_and_counts_what_landed() {
    let server = Server::start(vec![gzip_scene()]);
    let mut sink = Cursor::new(Vec::new());
    let response = agent_of(&server).stream(&request_read(), &mut sink).expect("an answer");

    assert_eq!(response.streamed_count, GZIP_UNPACKED_BYTES, "the size Jira reports for the file");
    assert_eq!(size::of_slice(sink.get_ref()), GZIP_UNPACKED_BYTES, "the sink agrees");
    assert_eq!(server.finish().len(), 1, "one request was made");
}

#[test]
fn a_compressed_read_with_nothing_to_unpack_is_a_failure_rather_than_an_empty_success() {
    let framed = "HTTP/1.1 200 OK\r\nContent-Encoding: gzip\r\n";
    let sized = "Content-Length: 1000\r\nConnection: close\r\n\r\n";
    let chunked = "Transfer-Encoding: chunked\r\nConnection: close\r\n\r\n";
    let chunk = [b"a\r\n", GZIP_HEAD.get(..10).unwrap_or_default()].concat();

    for (tail, body) in [
        (sized, &[][..]),
        (chunked, &[][..]),
        (sized, GZIP_HEAD.get(..5).unwrap_or_default()),
        (sized, GZIP_HEAD.get(..10).unwrap_or_default()),
        (chunked, chunk.as_slice()),
        ("Content-Length: 0\r\nConnection: close\r\n\r\n", &[][..]),
        ("Connection: close\r\n\r\n", &[][..]),
    ] {
        let cut = Scene::Say([framed.as_bytes(), tail.as_bytes(), body].concat());
        let server = Server::start(vec![cut; 5]);
        let client = client_of(&server);
        let failure = client.send(&GetServerInfo).expect_err("nothing can be read");

        assert!(matches!(failure, Error::Transport(_)), "{tail:?} read as {failure:?}");
        assert_eq!(waits_of(&client).len(), 4, "a cut is retried");
        assert_eq!(server.finish().len(), 5, "and asked for every time");
    }
}

#[test]
fn a_connection_nobody_answers_is_a_transport_failure() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("a free port");
    let address = listener.local_addr().expect("a bound port");

    drop(listener);

    let agent = Agent::over(Loopback, Local { address });
    let failure = agent.fetch(&request_read(), BYTES_MAX).expect_err("a failure");

    assert!(matches!(failure, Error::Transport(_)), "the wire failed: {failure:?}");
}

#[test]
fn a_delete_jira_refuses_without_a_type_names_one_and_sends_no_body() {
    let server = Server::start(vec![answer(204, &[], "")]);
    let client = client_of(&server);

    let called =
        DeleteRemoteIssueLinkByGlobalID { global_id: "a=b&id=1", issue_id_or_key: "KAN-1" };

    let reply = client.send(&called).expect("a delete");
    let seen = server.finish();
    let request = seen[0].to_lowercase();

    assert_eq!(reply.status(), 204, "the delete landed");

    assert!(request.starts_with("delete "), "the method survives: {request}");
    assert!(request.contains("content-type: application/json"), "the type is sent: {request}");
    assert!(seen[0].contains("globalId=a%3Db%26id%3D1"), "the id is escaped: {}", seen[0]);
    assert!(seen[0].ends_with("\r\n\r\n"), "no body follows the headers: {}", seen[0]);
}

#[test]
fn a_dot_segment_never_climbs_out_of_the_path_into_the_host() {
    let moved = answer(302, &[("Location", "HTTPS://api.atlassian.com/../evil.example/x")], "");
    let server = Server::start(vec![moved, answer(200, &[], "landed")]);
    let response = agent_of(&server).fetch(&request_read(), BYTES_MAX).expect("an answer");
    let seen = server.finish();
    let hop = seen[1].to_lowercase();

    assert_eq!(response.body, b"landed", "the hop was followed");
    assert!(hop.starts_with("get /evil.example/x"), "a path it stayed: {hop}");
    assert!(hop.contains("host: api.atlassian.com"), "on the host it named: {hop}");
    assert!(hop.contains("authorization: basic"), "which is its own origin: {hop}");
}

#[test]
fn a_download_answered_with_a_part_it_never_asked_for_is_no_download() {
    let part = [("Content-Range", "bytes 0-6/1000")];
    let server = Server::start(vec![answer(206, &part, "partial"), answer(206, &part, "partial")]);
    let mut sink = Cursor::new(Vec::new());

    let failure = client_of(&server)
        .download(&GetAttachmentContent { id: "9" }, &mut sink)
        .expect_err("seven bytes of a thousand are not the file");

    assert!(matches!(failure, Error::Reply(_)), "a fragment read as {failure:?}");
    assert!(sink.get_ref().is_empty(), "and never reached the sink");

    let mut ranged = request_read();

    ranged.header("Range", "bytes=0-6");

    let response = agent_of(&server).stream(&ranged, &mut sink).expect("a part it asked for");

    assert_eq!(response.streamed_count, 7, "a part is the file when it was the request");
    assert_eq!(server.finish().len(), 2, "each answer was asked for once");
}

#[test]
fn a_download_answered_with_no_content_lands_nothing_and_says_so() {
    let server = Server::start(vec![answer(204, &[], "")]);
    let mut sink = Cursor::new(Vec::new());

    let written =
        client_of(&server).download(&GetAttachmentContent { id: "9" }, &mut sink).expect("a 204");

    assert_eq!(written, 0, "nothing landed");
    assert!(sink.into_inner().is_empty(), "and nothing was written");
    assert_eq!(server.finish().len(), 1, "one request was made");
}

#[test]
fn a_download_answered_with_nothing_and_no_file_is_no_download() {
    let cut = said("HTTP/1.1 200 OK\r\nContent-Length: 90\r\nConnection: close\r\n\r\nstale");
    let server = Server::start(vec![cut, answer(202, &[], "")]);
    let mut sink = Cursor::new(Vec::new());

    let failure = client_of(&server)
        .download(&GetAttachmentContent { id: "9" }, &mut sink)
        .expect_err("an empty 202 holds no file");

    assert!(matches!(failure, Error::Reply(_)), "it is not a file of no bytes: {failure:?}");
    assert_eq!(server.finish().len(), 2, "the second answer ended it");
}

#[test]
fn a_download_answered_with_something_other_than_the_file_is_no_download() {
    let server = Server::start(vec![answer(202, &[], "{\"status\":\"preparing\"}")]);
    let mut sink = Cursor::new(Vec::new());

    let failure = client_of(&server)
        .download(&GetAttachmentContent { id: "9" }, &mut sink)
        .expect_err("a 202 holds no file");

    assert!(matches!(failure, Error::Reply(_)), "the answer is not the file: {failure:?}");
    assert!(sink.into_inner().is_empty(), "and it never reached the sink");
    assert_eq!(server.finish().len(), 1, "one request was made");
}

#[test]
fn a_download_cut_short_starts_over_and_lands_whole() {
    let partial = said("HTTP/1.1 200 OK\r\nContent-Length: 90\r\nConnection: close\r\n\r\npartial");
    let server = Server::start(vec![partial, answer(200, &[], "the whole body")]);
    let client = client_of(&server);
    let mut sink = Cursor::new(Vec::new());

    let written =
        client.download(&GetAttachmentContent { id: "9" }, &mut sink).expect("a download");

    assert_eq!(written, 14, "the second attempt's length is the one reported");
    assert_eq!(sink.into_inner(), b"the whole body", "the sink holds the second attempt alone");
    assert_eq!(waits_of(&client).len(), 1, "one retry");
    assert_eq!(server.finish().len(), 2, "the download ran twice");
}

#[test]
fn a_download_keeps_what_the_sink_held_before_it() {
    let cut = said("HTTP/1.1 200 OK\r\nContent-Length: 90\r\nConnection: close\r\n\r\npartial");
    let server = Server::start(vec![cut, answer(200, &[], "whole body")]);
    let client = client_of(&server);
    let mut sink = Cursor::new(b"header:".to_vec());

    let _end = sink.seek(io::SeekFrom::End(0)).expect("the end of the sink");

    let written =
        client.download(&GetAttachmentContent { id: "9" }, &mut sink).expect("a download");

    assert_eq!(written, 10, "the length of the body alone");
    assert_eq!(sink.into_inner(), b"header:whole body", "the retry starts where the first did");
    assert_eq!(server.finish().len(), 2, "the download ran twice");
}

#[test]
fn a_download_lands_in_its_sink_and_never_in_memory() {
    let server = Server::start(vec![answer(200, &[], "the attachment")]);
    let mut sink = Cursor::new(Vec::new());
    let response = agent_of(&server).stream(&request_read(), &mut sink).expect("an answer");

    assert_eq!(response.streamed_count, 14, "every byte is counted");
    assert!(response.body.is_empty(), "nothing is held in memory");
    assert_eq!(sink.into_inner(), b"the attachment", "the sink holds the body");
    assert_eq!(server.finish().len(), 1, "one request was made");
}

#[test]
fn a_download_retried_shorter_reports_the_length_that_counts() {
    let cut =
        said("HTTP/1.1 200 OK\r\nContent-Length: 90\r\nConnection: close\r\n\r\n0123456789abcdef");

    let server = Server::start(vec![cut, answer(200, &[], "short")]);
    let client = client_of(&server);
    let mut sink = Cursor::new(Vec::new());

    let written =
        client.download(&GetAttachmentContent { id: "9" }, &mut sink).expect("a download");

    let held = sink.into_inner();

    assert_eq!(written, 5, "the count is the whole of the file");
    assert_eq!(&held[..5], b"short", "and it covers the retry");
    assert_eq!(&held[5..], b"56789abcdef", "a sink cannot be shortened, so the tail stays");
    assert_eq!(server.finish().len(), 2, "the download ran twice");
}

#[test]
fn a_download_that_stalls_ends_at_its_idle_limit() {
    let stall = Scene::Stall(b"HTTP/1.1 200 OK\r\nContent-Length: 90\r\n\r\npartial".to_vec());
    let server = Server::start(vec![stall]);
    let idle = Duration::from_millis(250);
    let agent = Agent::over_idle(Loopback, Local { address: server.address }, idle);
    let mut sink = Cursor::new(Vec::new());
    let failure = agent.stream(&request_read(), &mut sink).expect_err("a stalled link");

    assert!(matches!(failure, Error::Transport(_)), "the wire failed: {failure:?}");
    assert_eq!(sink.into_inner(), b"partial", "what arrived before the stall was written");
    assert_eq!(server.finish().len(), 1, "one request was made");
}

#[test]
fn a_get_follows_a_redirect_and_signs_only_its_own_origin() {
    for (target, is_signed) in [
        ("https://api.media.atlassian.com/file/1/binary", false),
        ("https://api.atlassian.com.evil.example/file", false),
        ("https://api.atlassian.com:8443/file", false),
        ("//api.media.atlassian.com/file", false),
        ("https://api.atlassian.com/rest/api/3/moved", true),
        ("/rest/api/3/moved", true),
        ("moved?a=1#top", true),
    ] {
        let moved = answer(302, &[("Location", target)], "");
        let server = Server::start(vec![moved, answer(200, &[], "the file")]);
        let response = agent_of(&server).fetch(&request_read(), BYTES_MAX).expect("an answer");
        let seen = server.finish();

        assert_eq!(response.body, b"the file", "the redirect was followed to {target}");
        assert_eq!(seen.len(), 2, "two requests were made");
        assert!(seen[0].to_lowercase().contains("authorization: basic"), "the first is signed");

        assert_eq!(
            seen[1].to_lowercase().contains("authorization"),
            is_signed,
            "the credential goes only where it already went: {target}",
        );
    }
}

#[test]
fn a_landed_write_cut_short_stays_a_success() {
    let cut =
        "HTTP/1.1 201 Created\r\nContent-Length: 90\r\nConnection: close\r\n\r\n{\"id\":\"1\"";

    let server = Server::start(vec![said(cut)]);
    let body = json!({"fields": {}});
    let reply = client_of(&server).send(&CreateIssue { body: &body }).expect("the write landed");

    assert_eq!(reply.status(), 201, "Jira said so before the body stopped");
    assert_eq!(reply.value(), "{\"id\":\"1\"", "and this much of the answer is kept");
    assert_eq!(server.finish().len(), 1, "nothing was repeated");
}

#[test]
fn a_landed_write_past_the_reply_limit_stays_a_success() {
    let server = Server::start(vec![answer(201, &[], "{\"key\":\"A-1\"}")]);
    let response = agent_of(&server).fetch(&request_write(), 4).expect("the write landed");

    assert_eq!(response.status, 201, "Jira said so");
    assert_eq!(response.body, b"{\"ke", "and what fits is kept");
    assert_eq!(server.finish().len(), 1, "nothing was repeated");
}

#[test]
fn a_rate_limit_waits_what_the_server_asked_for() {
    let limited = answer(429, &[("Retry-After", "7")], "");
    let server = Server::start(vec![limited, answer(200, &[], "{}")]);
    let client = client_of(&server);
    let reply = client.send(&GetServerInfo).expect("the second attempt lands");
    let waits = waits_of(&client);

    assert!(reply.value().is_object(), "the reply is the second answer");
    assert_eq!(waits.len(), 1, "one wait");
    assert!(waits[0] >= Duration::from_secs(7), "the wait honors the server: {:?}", waits[0]);
    assert_eq!(server.finish().len(), 2, "the request ran twice");
}

#[test]
fn a_redirect_cut_after_its_last_chunk_is_followed_rather_than_a_crash() {
    let cut =
        said("HTTP/1.1 302 Found\r\nLocation: /next\r\nTransfer-Encoding: chunked\r\n\r\n0\r\n");

    let server = Server::start(vec![cut, answer(200, &[], "landed")]);
    let response = agent_of(&server).fetch(&request_read(), BYTES_MAX).expect("an answer");

    assert_eq!(response.body, b"landed", "a body nobody needs is never read");
    assert_eq!(server.finish().len(), 2, "the redirect was followed");
}

#[test]
fn a_redirect_loop_stops_at_its_budget() {
    let scenes = vec![answer(302, &[("Location", "/again")], ""); 12];
    let server = Server::start(scenes);
    let failure = agent_of(&server).fetch(&request_read(), BYTES_MAX).expect_err("a failure");
    let seen = server.finish();

    assert!(matches!(failure, Error::Reply(_)), "the wire worked, the answer did not: {failure:?}");
    assert!(failure.message().contains("too many redirects"), "the failure says why");

    assert_eq!(
        u32::try_from(seen.len()).ok(),
        REDIRECT_COUNT_MAX.checked_add(1),
        "the first request and its budget of redirects",
    );
}

#[test]
fn a_redirect_on_a_write_is_never_followed_and_may_have_landed() {
    for status in [301, 302, 307, 308] {
        let moved = answer(status, &[("Location", "/elsewhere")], "");
        let server = Server::start(vec![moved]);
        let body = json!({"fields": {}});
        let failure = client_of(&server).send(&CreateIssue { body: &body }).expect_err("not a 303");

        assert!(matches!(failure, Error::Indefinite(_)), "{status} read as {failure:?}");
        assert_eq!(server.finish().len(), 1, "the redirect was not followed");
    }
}

#[test]
fn a_redirect_that_names_nowhere_is_a_fault_of_the_reply() {
    for scene in [
        answer(302, &[], ""),
        answer(302, &[("Location", "")], ""),
        answer(302, &[("Location", "https://[")], ""),
        said("HTTP/1.1 302 Found\r\nLocation: /a b\r\nContent-Length: 0\r\n\r\n"),
    ] {
        let server = Server::start(vec![scene]);
        let client = client_of(&server);
        let failure = client.send(&GetServerInfo).expect_err("nowhere to go");

        assert!(matches!(failure, Error::Reply(_)), "the wire worked: {failure:?}");
        assert!(waits_of(&client).is_empty(), "the same answer would come back");
        assert_eq!(server.finish().len(), 1, "one request was made");
    }
}

#[test]
fn a_redirect_that_names_only_a_fragment_keeps_the_page_it_asked_for() {
    let moved = answer(302, &[("Location", "#top")], "");
    let server = Server::start(vec![moved, answer(200, &[], "{\"values\":[]}")]);
    let client = client_of(&server);
    let pages: Vec<_> = client.walk_with(&SearchProjects, search_from(50)).collect();
    let seen = server.finish();

    assert!(pages.iter().all(Result::is_ok), "the walk reads");
    assert!(seen[1].contains("startAt=50"), "the second hop keeps its query: {}", seen[1]);
}

#[test]
fn a_redirect_to_an_uppercase_scheme_is_followed_and_can_redirect_again() {
    let shouted = answer(302, &[("Location", "HTTPS://api.atlassian.com/rest/api/3/moved")], "");
    let again = answer(302, &[("Location", "/rest/api/3/there")], "");
    let server = Server::start(vec![shouted, again, answer(200, &[], "landed")]);
    let response = agent_of(&server).fetch(&request_read(), BYTES_MAX).expect("an answer");
    let seen = server.finish();

    assert_eq!(response.body, b"landed", "a scheme is the same in any case");
    assert!(seen[1].starts_with("GET /rest/api/3/moved"), "the hop went: {}", seen[1]);
    assert!(seen[2].starts_with("GET /rest/api/3/there"), "and so did the next: {}", seen[2]);
    assert!(seen[2].to_lowercase().contains("authorization: basic"), "on its own origin");
}

#[test]
fn a_redirect_to_plain_http_is_refused_before_it_connects() {
    let moved = answer(302, &[("Location", "http://api.atlassian.com/plain")], "");
    let server = Server::start(vec![moved, answer(200, &[], "never sent")]);
    let failure = agent_of(&server).fetch(&request_read(), BYTES_MAX).expect_err("a refusal");

    assert!(matches!(failure, Error::Reply(_)), "a retry would be sent the same way: {failure:?}");
    assert!(failure.message().contains("https only"), "the failure says why");
    assert_eq!(server.finish().len(), 1, "the plain address was never reached");
}

#[test]
fn a_refusal_cut_short_keeps_its_status() {
    let cut = "HTTP/1.1 404 Not Found\r\nContent-Length: 90\r\nConnection: close\r\n\r\ngone";
    let server = Server::start(vec![said(cut)]);
    let client = client_of(&server);
    let failure = client.send(&GetServerInfo).expect_err("a refusal");

    assert_eq!(failure.status(), Some(404), "the status arrived before the body stopped");
    assert!(failure.message().contains("gone"), "and so did this much of it");
    assert!(waits_of(&client).is_empty(), "a 404 is never retried");
    assert_eq!(server.finish().len(), 1, "one request was made");
}

#[test]
fn a_refusal_is_read_rather_than_raised() {
    for status in [400, 401, 404, 429, 500, 503] {
        let server = Server::start(vec![answer(status, &[], "refused")]);
        let response = agent_of(&server).fetch(&request_read(), BYTES_MAX).expect("an answer");

        assert_eq!(response.status, status, "the status arrives");
        assert_eq!(response.body, b"refused", "the body of a refusal arrives");
        assert_eq!(server.finish().len(), 1, "one request was made");
    }
}

#[test]
fn a_refused_download_cut_short_keeps_its_status() {
    let partial =
        said("HTTP/1.1 404 Not Found\r\nContent-Length: 90\r\nConnection: close\r\n\r\ngone");

    let server = Server::start(vec![partial]);
    let mut sink = Cursor::new(Vec::new());
    let response = agent_of(&server).stream(&request_read(), &mut sink).expect("the status came");
    let text = String::from_utf8_lossy(&response.body).into_owned();

    assert_eq!(response.status, 404, "a lost body never hides the refusal");
    assert!(text.starts_with("gone"), "what did arrive is kept: {text}");
    assert!(text.contains("the rest was lost"), "the message says the body was cut: {text}");
    assert!(sink.into_inner().is_empty(), "an error page never lands in a file");
    assert_eq!(server.finish().len(), 1, "one request was made");
}

#[test]
fn a_refused_download_holds_no_more_than_its_excerpt() {
    let page = "x".repeat(4_096);
    let server = Server::start(vec![answer(404, &[], &page)]);
    let mut sink = Cursor::new(Vec::new());
    let response = agent_of(&server).stream(&request_read(), &mut sink).expect("an answer");

    assert_eq!(response.status, 404, "a long error page keeps its status");
    assert_eq!(size::of_slice(&response.body), limits::ERROR_EXCERPT_BYTES_MAX, "a bounded read");
    assert_eq!(server.finish().len(), 1, "one request was made");
}

#[test]
fn a_refused_download_keeps_what_jira_said() {
    let said = "{\"errorMessages\":[\"no such attachment\"]}";
    let server = Server::start(vec![answer(404, &[], said)]);
    let client = client_of(&server);
    let mut sink = Cursor::new(Vec::new());
    let called = GetAttachmentContent { id: "9" };
    let failure = client.download(&called, &mut sink).expect_err("a refusal");

    assert!(matches!(failure, Error::Jira { status: 404, message: _ }), "refused: {failure:?}");
    assert!(failure.message().contains("no such attachment"), "the reason survives: {failure:?}");
    assert!(sink.into_inner().is_empty(), "an error page never lands in a file");
    assert_eq!(server.finish().len(), 1, "a refusal is not retried");
}

#[test]
fn a_refused_download_with_no_body_keeps_its_status() {
    let server = Server::start(vec![answer(404, &[], "")]);
    let mut sink = Cursor::new(Vec::new());

    let failure = client_of(&server)
        .download(&GetAttachmentContent { id: "9" }, &mut sink)
        .expect_err("a refusal");

    assert_eq!(failure.status(), Some(404), "a refusal stays one with nothing in it");
    assert_eq!(server.finish().len(), 1, "one request was made");
}

#[test]
fn a_refused_download_writes_nothing() {
    let server = Server::start(vec![answer(404, &[], "the error page")]);
    let mut sink = Cursor::new(Vec::new());
    let response = agent_of(&server).stream(&request_read(), &mut sink).expect("an answer");

    assert_eq!(response.status, 404, "a refusal is read");
    assert_eq!(response.streamed_count, 0, "nothing is counted");
    assert!(sink.into_inner().is_empty(), "an error page never lands in a file");
    assert_eq!(server.finish().len(), 1, "one request was made");
}

#[test]
fn a_refused_write_cut_short_is_a_refusal_rather_than_a_write_that_may_have_landed() {
    let cut = "HTTP/1.1 400 Bad\r\nContent-Length: 90\r\nConnection: close\r\n\r\nno";
    let server = Server::start(vec![said(cut)]);
    let body = json!({"fields": {}});
    let failure = client_of(&server).send(&CreateIssue { body: &body }).expect_err("a refusal");

    assert_eq!(failure.status(), Some(400), "the write never landed");
    assert_eq!(server.finish().len(), 1, "one request was made");
}

#[test]
fn a_server_error_is_read_again_until_it_answers() {
    let failing = answer(503, &[], "busy");
    let server = Server::start(vec![failing.clone(), failing, answer(200, &[], "{}")]);
    let client = client_of(&server);
    let reply = client.send(&GetServerInfo).expect("the third attempt lands");

    assert!(reply.value().is_object(), "the reply is the last answer");
    assert_eq!(waits_of(&client).len(), 2, "two waits");
    assert_eq!(server.finish().len(), 3, "the read ran three times");
}

#[test]
fn a_sink_that_refuses_a_download_is_not_a_network_failure() {
    let server = Server::start(vec![answer(200, &[], "the attachment")]);
    let client = client_of(&server);
    let operation = GetAttachmentContent { id: "9" };
    let failure = client.download(&operation, &mut SinkRefusing).expect_err("a refusal");

    assert!(matches!(failure, Error::Input(_)), "the disk is not the wire: {failure:?}");
    assert!(waits_of(&client).is_empty(), "no retry");
    assert_eq!(server.finish().len(), 1, "a full disk is never asked twice");
}

#[test]
fn a_status_at_the_top_of_the_range_is_a_status() {
    let server = Server::start(vec![answer(599, &[], "odd"); 5]);
    let failure = client_of(&server).send(&GetServerInfo).expect_err("a refusal");

    assert_eq!(failure.status(), Some(599), "the last real status stays a status");
    assert_eq!(server.finish().len(), 5, "and is retried as a server error");
}

#[test]
fn a_status_past_the_range_is_a_fault_of_the_reply() {
    let server = Server::start(vec![said("HTTP/1.1 999 Odd\r\nContent-Length: 0\r\n\r\n")]);
    let client = client_of(&server);
    let failure = client.send(&GetServerInfo).expect_err("an impossible status");

    assert!(matches!(failure, Error::Reply(_)), "the wire worked: {failure:?}");
    assert!(waits_of(&client).is_empty(), "the same answer would come back");
    assert_eq!(server.finish().len(), 1, "one request was made");
}

#[test]
fn a_success_labelled_compressed_with_no_body_stays_a_success() {
    let none = "HTTP/1.1 204 No Content\r\nContent-Encoding: gzip\r\nConnection: close\r\n\r\n";
    let empty = "HTTP/1.1 200 OK\r\nContent-Encoding: gzip\r\nContent-Length: 0\r\n\r\n";
    let server = Server::start(vec![said(none), said(none), said(empty)]);
    let client = client_of(&server);
    let read = client.send(&GetServerInfo).expect("a 204 holds nothing to unpack");
    let mut sink = Cursor::new(Vec::new());
    let written = client.download(&GetAttachmentContent { id: "9" }, &mut sink).expect("a 204");
    let write = client.send(&ArchiveProject { project_id_or_key: "SWP" }).expect("a landed write");

    assert_eq!(read.value(), &serde_json::Value::Null, "nothing is nothing");
    assert_eq!(written, 0, "and nothing landed in the sink");
    assert_eq!(write.status(), 200, "and a success stays one");
    assert!(waits_of(&client).is_empty(), "none of them was retried");
    assert_eq!(server.finish().len(), 3, "each ran once");
}

#[test]
fn a_text_reply_jira_left_unquoted_still_reads() {
    let headers = [("Content-Type", "application/json;charset=UTF-8")];
    let server = Server::start(vec![answer(200, &headers, "KAN1")]);
    let client = client_of(&server);
    let options = get_valid_project_key::Options { key: Some("KAN") };
    let reply = client.send_with(&GetValidProjectKey, &options).expect("a success is a success");

    assert_eq!(reply.read(), Some("KAN1"), "the bare text is the reply");
    assert!(server.finish()[0].contains("key=KAN"), "the option reaches the wire");
}

#[test]
fn a_throttled_write_cut_short_is_still_retried() {
    let cut =
        "HTTP/1.1 429 Slow\r\nRetry-After: 1\r\nContent-Length: 90\r\nConnection: close\r\n\r\nno";

    let server = Server::start(vec![said(cut), answer(201, &[], "{\"key\":\"A-1\"}")]);
    let client = client_of(&server);
    let body = json!({"fields": {}});
    let reply = client.send(&CreateIssue { body: &body }).expect("the second attempt landed");

    assert_eq!(reply.value()["key"], "A-1", "a 429 never landed, whatever its body did");
    assert_eq!(waits_of(&client).len(), 1, "one wait");
    assert_eq!(server.finish().len(), 2, "the write ran twice");
}

#[test]
fn a_timeout_ends_a_call_that_never_answers() {
    let server = Server::start(vec![Scene::Hang]);
    let agent = agent_of(&server);
    let failure = agent.call(&request_read(), Duration::from_millis(250)).expect_err("a timeout");

    assert!(matches!(failure, Error::Transport(_)), "the wire failed: {failure:?}");
    assert!(failure.message().contains("timeout"), "the failure says why");
    assert_eq!(server.finish().len(), 1, "the request arrived and was never answered");
}

#[test]
fn a_walk_crosses_its_pages_over_the_wire() {
    let first = json!({"values": [{"key": "A"}], "isLast": false}).to_string();
    let second = json!({"values": [{"key": "B"}], "isLast": true}).to_string();
    let server = Server::start(vec![answer(200, &[], &first), answer(200, &[], &second)]);
    let client = client_of(&server);

    let pages: Vec<_> =
        client.walk(&SearchProjects).map(|page| page.expect("a page").count()).collect();

    let seen = server.finish();

    assert_eq!(pages, [1, 1], "each page holds one project");
    assert!(seen[1].contains("startAt=1"), "the second page resumes past the first: {}", seen[1]);
}

#[test]
fn a_write_keeps_a_redirect_for_its_caller() {
    let moved = answer(303, &[("Location", "https://api.atlassian.com/task/1")], "");
    let server = Server::start(vec![moved, answer(200, &[], "never sent")]);
    let mut request = request_write();

    request.body_json(&json!({"fields": {}})).expect("a body");

    let response = agent_of(&server).fetch(&request, BYTES_MAX).expect("an answer");

    assert_eq!(response.status, 303, "a write is not redirected");
    assert_eq!(response.location.as_deref(), Some("https://api.atlassian.com/task/1"), "kept");
    assert_eq!(server.finish().len(), 1, "the address was never followed");
}

#[test]
fn a_write_that_lost_its_connection_is_never_sent_again() {
    let server = Server::start(vec![Scene::Vanish, answer(201, &[], "{}")]);
    let client = client_of(&server);
    let body = json!({"fields": {"summary": "once"}});
    let failure = client.send(&CreateIssue { body: &body }).expect_err("an unknown outcome");

    assert!(matches!(failure, Error::Indefinite(_)), "it may have landed: {failure:?}");
    assert!(waits_of(&client).is_empty(), "no retry was planned");
    assert_eq!(server.finish().len(), 1, "a write that may have landed is sent once");
}

fn agent_of(server: &Server) -> Agent {
    assert_ne!(server.address.port(), 0, "a server is bound");

    Agent::over(Loopback, Local { address: server.address })
}

#[test]
fn an_attachment_upload_reaches_the_wire_as_one_multipart_body() {
    let reply = json!([{"id": "1"}]).to_string();
    let server = Server::start(vec![answer(200, &[], &reply)]);
    let client = client_of(&server);

    let part = Part {
        bytes: b"attachment text",
        content_type: "text/plain",
        file_name: Some("notes.txt"),
        name: "file",
    };

    let parts = [part];
    let found = client.send(&AddAttachment { body: &parts, issue_id_or_key: "ABC-1" });
    let seen = server.finish();
    let request = seen[0].to_lowercase();

    assert!(found.is_ok(), "the upload lands: {found:?}");
    assert!(request.contains("x-atlassian-token: no-check"), "the check is waived");
    assert!(request.contains("content-type: multipart/form-data; boundary="), "a form");
    assert!(request.contains("filename=\"notes.txt\""), "the part names its file");
    assert!(request.contains("attachment text"), "the bytes travel");
}

fn answer(status: u16, headers: &[(&str, &str)], body: &str) -> Scene {
    assert!((100..=599).contains(&status), "a scene answers with a real status");

    let mut text = format!(
        "HTTP/1.1 {status} Scene\r\nConnection: close\r\nContent-Length: {}\r\n",
        body.len(),
    );

    for (name, value) in headers {
        text.push_str(name);
        text.push_str(": ");
        text.push_str(value);
        text.push_str("\r\n");
    }

    text.push_str("\r\n");
    text.push_str(body);

    Scene::Say(text.into_bytes())
}

fn client_of(server: &Server) -> Client<Instant> {
    let network = Instant { agent: agent_of(server), waits: Mutex::new(Vec::new()) };

    Client::new(network, &CREDENTIALS).expect("the fixture credentials")
}

fn connection_accept(listener: &TcpListener, is_finished: &AtomicBool) -> Option<TcpStream> {
    for _poll_index in 0..POLL_COUNT_MAX {
        match listener.accept() {
            Ok((stream, _peer)) => {
                stream.set_nonblocking(false).expect("a blocking stream");

                return Some(stream);
            }
            Err(cause) => {
                assert_eq!(cause.kind(), io::ErrorKind::WouldBlock, "a poll only ever waits");
            }
        }

        if is_finished.load(Ordering::SeqCst) {
            return None;
        }

        thread::park_timeout(Duration::from_millis(POLL_MILLIS));
    }

    None
}

#[test]
fn every_signal_of_a_rate_limit_survives_the_wire() {
    let headers = [
        ("Retry-After", "31"),
        ("RateLimit-Reason", "jira-burst-based"),
        ("X-RateLimit-NearLimit", "true"),
    ];

    let server = Server::start(vec![answer(429, &headers, "")]);
    let response = agent_of(&server).fetch(&request_read(), BYTES_MAX).expect("an answer");

    assert_eq!(response.status, 429, "the status");
    assert_eq!(response.retry_after_seconds, Some(31), "the delay");
    assert_eq!(response.rate_limit_reason.as_deref(), Some("jira-burst-based"), "the reason");
    assert!(response.is_near_limit, "the flag");
    assert_eq!(server.finish().len(), 1, "one request was made");
}

fn gzip_scene() -> Scene {
    let body = [GZIP_HEAD, &[0; GZIP_ZERO_RUN_BYTES], GZIP_TAIL].concat();

    let head = concat!(
        "HTTP/1.1 200 OK\r\nContent-Encoding: gzip\r\n",
        "Connection: close\r\nContent-Length: ",
    );

    let tail = format!("{}\r\n\r\n", body.len());

    Scene::Say([head.as_bytes(), tail.as_bytes(), &body].concat())
}

#[test]
fn hostile_bytes_through_the_real_stack_end_in_an_error_and_never_in_a_panic() {
    let seed = env::var("WIRE_FUZZ_SEED").ok().and_then(|text| text.parse().ok()).unwrap_or(1);
    let cases = env::var("WIRE_FUZZ_CASES").ok().and_then(|text| text.parse().ok());
    let mut prng = Prng::from_seed(seed);

    for case_index in 0..cases.unwrap_or(HOSTILE_CASES_DEFAULT) {
        hostile_case(&mut prng, case_index.rem_euclid(4));
    }
}

fn hostile_case(prng: &mut Prng, mode: u64) {
    assert!(mode < 4, "a case is a read, a download, a write, or a walk");

    let writes_sent = Arc::new(AtomicU64::new(0));
    let connection_count = prng.index(HOSTILE_CONNECTION_COUNT_MAX).strict_add(1);
    let mut connections = VecDeque::with_capacity(size::offset(connection_count));
    let mut status_first = "";

    for connection_index in 0..connection_count {
        let status = hostile_status(prng, mode);

        status_first = if connection_index == 0 { status } else { status_first };
        let response = hostile_response(prng, status);

        connections.push_back(hostile_pieces(prng, response));
    }

    let connector = MemoryConnector {
        connections: Mutex::new(connections),
        writes_sent: Arc::clone(&writes_sent),
    };

    let address = SocketAddr::from(([127, 0, 0, 1], 9));
    let agent = Agent::over(connector, Local { address });
    let network = Instant { agent, waits: Mutex::new(Vec::new()) };
    let client = Client::new(network, &CREDENTIALS).expect("a client");

    match mode {
        0 => drop(client.send(&GetServerInfo)),
        1 => drop(client.download(&GetAttachmentContent { id: "9" }, &mut Cursor::new(Vec::new()))),
        2 => drop(client.send(&CreateIssue { body: &json!({"fields": {}}) })),
        _ => drop(client.walk(&SearchProjects).take(8).count()),
    }

    let sent = writes_sent.load(Ordering::SeqCst);

    assert!(sent <= 5, "a write is sent at most five times: {sent}");
    assert!(sent <= 1 || status_first == "429", "only a 429 repeats a write: {status_first}");
}

fn hostile_pick<Item>(prng: &mut Prng, items: &[Item]) -> Item
where
    Item: Copy,
{
    assert!(!items.is_empty(), "a pick draws from at least one item");

    let index = size::offset(prng.index(size::of_slice(items)));

    *items.get(index).expect("a pick stays inside its items")
}

fn hostile_pieces(prng: &mut Prng, mut bytes: Vec<u8>) -> VecDeque<Vec<u8>> {
    assert!(!bytes.is_empty(), "an answer holds at least its status line");

    if prng.boolean() {
        return VecDeque::from([bytes]);
    }

    let tail = bytes.split_off(size::offset(prng.index(size::of_slice(&bytes))));

    VecDeque::from([bytes, tail]).into_iter().filter(|piece| !piece.is_empty()).collect()
}

fn hostile_response(prng: &mut Prng, status: &str) -> Vec<u8> {
    assert!(!status.is_empty(), "a response names its status");

    let version = hostile_pick(prng, &HOSTILE_VERSIONS);
    let prelude = hostile_pick(prng, &HOSTILE_PRELUDES);
    let mut head = format!("{prelude}{version} {status} Why\r\n");

    for (chance, name, value) in [
        (70, "Location", hostile_pick(prng, &HOSTILE_LOCATIONS)),
        (40, "Content-Length", hostile_pick(prng, &HOSTILE_LENGTHS)),
        (10, "Content-Length", hostile_pick(prng, &HOSTILE_LENGTHS)),
        (50, "Transfer-Encoding", hostile_pick(prng, &HOSTILE_TRANSFERS)),
        (25, "Content-Encoding", hostile_pick(prng, &HOSTILE_ENCODINGS)),
        (30, "Connection", hostile_pick(prng, &HOSTILE_CONNECTIONS)),
        (20, "Retry-After", hostile_pick(prng, &HOSTILE_RETRIES)),
    ] {
        if prng.chance(chance) {
            head.push_str(name);
            head.push_str(": ");
            head.push_str(value);
            head.push_str("\r\n");
        }
    }

    if prng.chance(90) {
        head.push_str("\r\n");
    }

    let mut response = head.into_bytes();

    for _body_index in 0..prng.index(3) {
        response.extend_from_slice(hostile_pick(prng, &HOSTILE_BODIES));
    }

    response
}

fn hostile_status(prng: &mut Prng, mode: u64) -> &'static str {
    assert!(mode < 4, "a case is a read, a download, a write, or a walk");

    if mode == 2 {
        return hostile_pick(prng, &HOSTILE_STATUSES);
    }

    if prng.boolean() {
        return hostile_pick(prng, &["301", "302", "303", "307", "308"]);
    }

    hostile_pick(prng, &HOSTILE_STATUSES)
}

fn request_arrived(bytes: &[u8]) -> bool {
    let text = String::from_utf8_lossy(bytes);

    let Some((head, body)) = text.split_once("\r\n\r\n") else {
        return false;
    };

    let length = head.lines().find_map(|line| {
        let lowered = line.to_lowercase();

        lowered.strip_prefix("content-length: ")?.trim().parse::<usize>().ok()
    });

    assert!(head.contains("HTTP/1.1"), "a request names its protocol");

    body.len() >= length.unwrap_or_default()
}

fn request_read() -> Request {
    let request = Request::of::<GetServerInfo>(URL_READ.to_owned(), AUTHORIZATION);

    assert!(request.is_read(), "a fixture read is a read");

    request
}

fn request_take(stream: &mut TcpStream) -> String {
    let mut bytes: Vec<u8> = Vec::new();
    let mut chunk = [0_u8; 4_096];

    for _read_index in 0..READ_COUNT_MAX {
        if request_arrived(&bytes) {
            break;
        }

        let count = stream.read(&mut chunk).unwrap_or_default();

        if count == 0 {
            break;
        }

        bytes.extend_from_slice(&chunk[..count]);
    }

    String::from_utf8_lossy(&bytes).into_owned()
}

fn request_write() -> Request {
    let body = json!({"fields": {}});
    let operation = CreateIssue { body: &body };
    let request = Request::of::<CreateIssue<'_>>(URL_READ.to_owned(), AUTHORIZATION);

    assert!(!request.is_read(), "a fixture write is a write");
    assert!(format!("{operation:?}").contains("CreateIssue"), "a fixture names its operation");

    request
}

fn said(text: &str) -> Scene {
    assert!(text.starts_with("HTTP/1.1 "), "a scene speaks HTTP");

    Scene::Say(text.as_bytes().to_vec())
}

fn scene_play(mut stream: TcpStream, scene: Scene) -> String {
    let timeout = Some(Duration::from_secs(READ_TIMEOUT_SECONDS));

    stream.set_read_timeout(timeout).expect("a read timeout");

    let request = request_take(&mut stream);

    match scene {
        Scene::Hang => {
            let _read = stream.read(&mut [0_u8; 1]);
        }
        Scene::Say(bytes) => {
            let _written = stream.write_all(&bytes);
        }
        Scene::Stall(bytes) => {
            let _written = stream.write_all(&bytes);
            let _read = stream.read(&mut [0_u8; 1]);
        }
        Scene::Vanish => {}
    }

    assert!(request.contains("HTTP/1.1"), "a scene plays to a request: {request}");

    request
}

fn scenes_play(
    listener: &TcpListener,
    scenes: Vec<Scene>,
    is_finished: &AtomicBool,
) -> Vec<String> {
    assert!(!scenes.is_empty(), "a server plays at least one scene");

    let mut seen = Vec::with_capacity(scenes.len());

    for scene in scenes {
        let Some(stream) = connection_accept(listener, is_finished) else {
            break;
        };

        seen.push(scene_play(stream, scene));
    }

    seen
}

fn search_from(start_at: i64) -> search_projects::Options<'static> {
    assert!(start_at >= 0, "a walk starts at or after the first item");

    search_projects::Options {
        action: None,
        category_id: None,
        expand: None,
        id: None,
        keys: None,
        order_by: None,
        properties: None,
        property_query: None,
        query: None,
        results_max: None,
        start_at: Some(start_at),
        status: None,
        type_key: None,
    }
}

#[test]
fn the_request_carries_everything_the_operation_put_in_it() {
    let server = Server::start(vec![answer(201, &[], "{}")]);
    let mut request = request_write();

    request.body_json(&json!({"fields": {"summary": "hello"}})).expect("a body");

    let response = agent_of(&server).fetch(&request, BYTES_MAX).expect("an answer");
    let seen = server.finish();
    let head = seen[0].to_lowercase();

    assert_eq!(response.status, 201, "the answer arrives");
    assert!(seen[0].starts_with("POST /ex/jira/site/rest/api/3/serverInfo HTTP/1.1"), "the line");
    assert!(head.contains("host: api.atlassian.com"), "the host");
    assert!(head.contains(&format!("authorization: {}", AUTHORIZATION.to_lowercase())), "signed");
    assert!(head.contains("accept: application/json"), "the accept header");
    assert!(head.contains("content-type: application/json"), "the content type");
    assert!(seen[0].contains("\"summary\":\"hello\""), "the body travels");
}

fn waits_of(client: &Client<Instant>) -> Vec<Duration> {
    let waits = client.network().waits.lock().expect("the waits").clone();

    assert!(waits.len() <= 16, "a test waits a few times");

    waits
}
