use core::marker::PhantomData;

use serde_json::Value;

use crate::error::Error;
use crate::kind::Kind;
use crate::limits;
use crate::request::Request;
use crate::response::Response;
use crate::size;

const CUT_ARRAY: &[u8] = b"[{\"type\":\"text\",\"text\":\"[cut: nested too deep to read]\"}]";
const CUT_OBJECT: &[u8] = b"{\"type\":\"text\",\"text\":\"[cut: nested too deep to read]\"}";
const CUT_COUNT_MAX: u32 = 1_024;
const DEPTH_KEPT_MAX: u32 = 120;
const PARSER_DEPTH_MAX: u32 = 128;
const PARSER_DEPTH_REFUSAL: &str = "recursion limit exceeded";
const _: () = assert!(DEPTH_KEPT_MAX + 2 < PARSER_DEPTH_MAX, "a cut reads back in the parser");

#[derive(Clone, Debug)]
pub struct Reply<Body> {
    body: PhantomData<Body>,
    location: Option<String>,
    status: u16,
    value: Value,
}

impl<Body> Reply<Body>
where
    Body: Kind,
{
    #[must_use]
    pub fn into_value(self) -> Value {
        assert!(self.status >= 200, "only a success reads as a reply");

        self.value
    }

    #[must_use]
    pub fn location(&self) -> Option<&str> {
        let location = self.location.as_deref();

        assert!(location.is_none_or(|found| !found.is_empty()), "a location names somewhere");

        location
    }

    pub(crate) fn parse(response: Response, request: &Request) -> Result<Self, Error> {
        assert!(request.url().starts_with("https://"), "every reply came over an encrypted link");
        assert!(response.status >= 200, "only a success reads as a reply");

        let value = value_read::<Body>(&response.body, request)?;

        let location = response.location.filter(|found| !found.is_empty());

        Ok(Self { body: PhantomData, location, status: response.status, value })
    }

    #[must_use]
    pub fn read(&self) -> Option<Body::Output<'_>> {
        assert!(self.status >= 200, "only a success reads as a reply");

        Body::read(&self.value)
    }

    #[must_use]
    pub const fn status(&self) -> u16 {
        assert!(self.status >= 200, "only a success reads as a reply");

        self.status
    }

    #[must_use]
    pub const fn value(&self) -> &Value {
        assert!(self.status >= 200, "only a success reads as a reply");

        &self.value
    }
}

fn depth_cut(body: &[u8]) -> Option<Vec<u8>> {
    assert!(!body.is_empty(), "a cut reads a body");

    let mut kept = Vec::with_capacity(body.len());
    let mut cut_at: Option<u32> = None;
    let mut cut_count: u32 = 0;
    let mut depth: u32 = 0;
    let mut is_escaped = false;
    let mut is_text = false;

    for byte in body.iter().copied() {
        let is_open = !is_text && matches!(byte, b'{' | b'[');
        let is_close = !is_text && matches!(byte, b'}' | b']');

        depth = depth.strict_add(u32::from(is_open));

        let starts_cut = is_open && cut_at.is_none() && depth > DEPTH_KEPT_MAX;

        if starts_cut {
            cut_at = Some(depth);
            cut_count = cut_count.strict_add(1);
            kept.extend_from_slice(if byte == b'{' { CUT_OBJECT } else { CUT_ARRAY });
        }

        if cut_count > CUT_COUNT_MAX {
            return None;
        }

        if cut_at.is_none() {
            kept.push(byte);
        }

        if is_close {
            cut_at = cut_at.filter(|at| *at != depth);
            depth = depth.saturating_sub(1);
        }

        (is_text, is_escaped) = match (is_text, is_escaped, byte) {
            (true, false, b'\\') => (true, true),
            (true, false, b'"') | (false, _, b'"') => (!is_text, false),
            (true, _, _) => (true, false),
            (false, _, _) => (false, false),
        };
    }

    assert!(cut_count <= CUT_COUNT_MAX, "a cut stays inside its budget");

    Some(kept)
}

pub(crate) fn json_read(body: &[u8]) -> Result<Value, serde_json::Error> {
    assert!(size::of_slice(body) <= limits::BODY_BYTES_MAX, "a body fits its limit");

    serde_json::from_slice(body).or_else(|cause| {
        if !cause.to_string().starts_with(PARSER_DEPTH_REFUSAL) {
            return Err(cause);
        }

        depth_cut(body).map_or(Err(cause), |kept| serde_json::from_slice(&kept))
    })
}

fn text_read(body: &[u8]) -> Value {
    assert!(!body.is_empty(), "a text reply says something");

    let text = serde_json::from_slice::<String>(body)
        .unwrap_or_else(|_bare| String::from_utf8_lossy(body).into_owned());

    assert!(text.len() <= body.len().strict_mul(3), "one byte lost is one replacement character");

    Value::String(text)
}

fn value_read<Body>(body: &[u8], request: &Request) -> Result<Value, Error>
where
    Body: Kind,
{
    assert!(!request.url().is_empty(), "a reply answers a request");

    if body.is_empty() {
        return Ok(Value::Null);
    }

    if Body::IS_BODY_TEXT {
        return Ok(text_read(body));
    }

    json_read(body).or_else(|cause| {
        if !request.is_read() {
            return Ok(text_read(body));
        }

        if !Body::IS_BODY_PROMISED {
            return Ok(text_read(body));
        }

        Err(Error::Reply(format!("invalid JSON from {}: {cause}", request.url())))
    })
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::{CUT_COUNT_MAX, DEPTH_KEPT_MAX, PARSER_DEPTH_MAX, Reply, depth_cut, json_read};
    use crate::error::Error;
    use crate::kind::{Any, Nothing, Text};
    use crate::request::tests::{read_request, write_request};
    use crate::response::Response;

    #[test]
    fn a_body_that_is_not_json_is_a_fault_of_the_reply_on_a_read() {
        let parsed = Reply::<Any>::parse(answer(b"<html>"), &read_request());

        assert!(matches!(parsed, Err(Error::Reply(_))), "the reply could not be read");

        let message = parsed.expect_err("a refused body").to_string();

        assert!(message.contains("/rest/api/3/"), "the failure names the request");
    }

    #[test]
    fn a_body_that_would_take_too_many_cuts_stays_a_failure_rather_than_a_memory_bomb() {
        let kept = usize::try_from(DEPTH_KEPT_MAX).expect("a small limit");
        let count = usize::try_from(CUT_COUNT_MAX).expect("a small limit");
        let open = "[".repeat(kept);
        let close = "]".repeat(kept);
        let deep = format!("{}{}", "[".repeat(40), "]".repeat(40));
        let inside = vec!["[]"; count.strict_sub(1)].join(",");
        let past = vec!["[]"; count].join(",");

        let fits = format!("{open}{inside},{deep}{close}");
        let over = format!("{open}{past},{deep}{close}");

        assert!(json_read(fits.as_bytes()).is_ok(), "a few cuts are fine");
        assert!(json_read(over.as_bytes()).is_err(), "and one too many refuses the body");
    }

    #[test]
    fn a_cut_keeps_the_kind_of_the_value_it_replaces() {
        let kept = usize::try_from(DEPTH_KEPT_MAX).expect("a small limit");
        let depth = kept.strict_mul(2);
        let arrays = format!("{}1{}", "[".repeat(depth), "]".repeat(depth));
        let objects = format!("{}1{}", "{\"a\":".repeat(depth), "}".repeat(depth));
        let cut = "{\"text\":\"[cut: nested too deep to read]\",\"type\":\"text\"}";

        let array_text = json_read(arrays.as_bytes()).expect("the arrays read").to_string();
        let object_text = json_read(objects.as_bytes()).expect("the objects read").to_string();

        assert!(array_text.starts_with(&format!("{}{cut}]", "[".repeat(kept.strict_add(1)))));
        assert!(object_text.starts_with(&format!("{}{cut}}}", "{\"a\":".repeat(kept))));
    }

    #[test]
    fn a_document_nested_past_the_parser_is_cut_there_and_the_rest_survives() {
        let depth = usize::try_from(PARSER_DEPTH_MAX).expect("a small limit").strict_mul(30);
        let deep = format!("{}\"bottom\"{}", "[".repeat(depth), "]".repeat(depth));
        let wide = "[".repeat(depth);
        let tail = "\"text\":\"[[[{\\\"\",\"after\":2}";
        let body = format!("{{\"before\":1,\"wide\":\"{wide}\",\"deep\":{deep},{tail}");
        let answered = answer_of(body.as_bytes());
        let reply = Reply::<Any>::parse(answered, &read_request()).expect("a page");
        let held = reply.value();

        assert!(serde_json::from_slice::<Value>(body.as_bytes()).is_err(), "the parser refuses it");
        assert_eq!(held["before"], 1_i64, "what comes before the cut is whole");
        assert_eq!(held["after"], 2_i64, "and so is what comes after it");
        assert_eq!(held["text"], "[[[{\"", "and a bracket inside a string is only text");
        assert_eq!(held["wide"].as_str(), Some(wide.as_str()), "however many of them there are");
        assert!(held.to_string().contains("[cut: nested too deep to read]"), "the cut says so");
        assert!(!held.to_string().contains("bottom"), "and what lay past it is gone");
    }

    #[test]
    fn a_landed_write_stays_a_success_whatever_its_body_holds() {
        let mut accepted = answer(b"<html>a redirect page</html>");

        accepted.location = Some("https://site/rest/api/3/task/10".to_owned());

        let reply = Reply::<Any>::parse(accepted, &write_request()).expect("the write landed");

        assert_eq!(reply.value(), &json!("<html>a redirect page</html>"), "kept as the text it is");
        assert_eq!(reply.location(), Some("https://site/rest/api/3/task/10"), "the task survives");
    }

    #[test]
    fn a_location_with_nothing_in_it_is_no_location() {
        let mut blank = answer(b"");

        blank.location = Some(String::new());

        let reply = Reply::<Any>::parse(blank, &read_request()).expect("a reply");

        assert_eq!(reply.location(), None, "a custom transport cannot reach the assertion");
    }

    #[test]
    fn a_reply_keeps_the_location_of_an_accepted_task() {
        let mut response = answer(b"");

        response.location = Some("https://site/rest/api/3/task/10".to_owned());

        let reply = Reply::<Any>::parse(response, &write_request()).expect("an accepted task");

        assert_eq!(reply.location(), Some("https://site/rest/api/3/task/10"), "the task");
        assert_eq!(reply.value(), &Value::Null, "with no body to read");
        assert_eq!(reply.read(), Some(&Value::Null), "which reads as a null");
    }

    #[test]
    fn a_reply_keeps_the_status_jira_answered() {
        let mut partial = answer(b"{\"entries\":[]}");

        partial.status = 207;

        let reply = Reply::<Nothing>::parse(partial, &write_request()).expect("a partial success");

        assert_eq!(reply.status(), 207, "a partial success is told from a whole one");

        let whole = Reply::<Any>::parse(answer(b""), &read_request()).expect("a whole success");

        assert_eq!(whole.status(), 200, "and a plain success says so");
    }

    #[test]
    fn a_reply_of_nothing_keeps_whatever_jira_sent() {
        let json = Reply::<Nothing>::parse(answer(b"{\"failed\":[1]}"), &write_request());
        let held = json.expect("a body the specification never promised");

        assert_eq!(held.read(), Some(()), "the type promises nothing");
        assert_eq!(held.value(), &json!({"failed": [1_i64]}), "and the value keeps what came");

        for request in [read_request(), write_request()] {
            let text = Reply::<Nothing>::parse(answer(b"<html>"), &request).expect("not a fault");

            assert_eq!(text.value(), &json!("<html>"), "text that is not JSON is kept as text");
            assert_eq!(text.read(), Some(()), "and the type still promises nothing");
            assert_eq!(text.into_value(), json!("<html>"), "owned");
        }

        let empty = Reply::<Nothing>::parse(answer(b""), &read_request()).expect("an empty reply");

        assert_eq!(empty.value(), &Value::Null, "nothing is nothing");
    }

    #[test]
    fn a_reply_reads_as_its_kind_and_keeps_its_value() {
        let reply = Reply::<Text>::parse(answer(b"\"words\""), &read_request()).expect("a text");

        assert_eq!(reply.read(), Some("words"), "typed");
        assert_eq!(reply.value(), &json!("words"), "raw");
        assert_eq!(reply.into_value(), json!("words"), "owned");
    }

    #[test]
    fn a_text_reply_reads_the_same_quoted_or_bare() {
        let bodies: [(&[u8], &str); 7] = [
            (b"\"VPNE\"", "VPNE"),
            (b"KAN1", "KAN1"),
            (b"Test 1", "Test 1"),
            (b"2024", "2024"),
            (b"true", "true"),
            (b"null", "null"),
            (b"\"\"", ""),
        ];

        for (body, text) in bodies {
            let reply = Reply::<Text>::parse(answer(body), &read_request()).expect("a text reply");

            assert_eq!(reply.read(), Some(text), "the body {body:?}");
            assert!(reply.value().is_string(), "a text reply holds a string");
        }
    }

    #[test]
    fn a_text_reply_that_is_not_text_at_all_is_kept_rather_than_a_crash() {
        let bodies: [(&[u8], &str); 5] = [
            (b"\"\\ud83d\"", "\"\\ud83d\""),
            (b"\xff", "\u{fffd}"),
            (b"\xff\xfe\xfd", "\u{fffd}\u{fffd}\u{fffd}"),
            (b"  \"\"  ", ""),
            (b"  \"a\"\n", "a"),
        ];

        for (body, text) in bodies {
            let reply = Reply::<Text>::parse(answer(body), &read_request()).expect("a text reply");

            assert_eq!(reply.read(), Some(text), "the body {body:?}");
        }
    }

    #[test]
    fn a_value_inside_the_limit_is_never_touched() {
        let depth = usize::try_from(DEPTH_KEPT_MAX).expect("a small limit");
        let inside = format!("{}1{}", "[".repeat(depth), "]".repeat(depth));
        let keyed = "{\"a\\\\\":[\"\\\"]\",{\"b\":null}],\"c\":[]}";

        for body in [inside.as_str(), keyed, "[]", "{}", "\"[\""] {
            assert_eq!(depth_cut(body.as_bytes()).as_deref(), Some(body.as_bytes()), "{body}");
        }

        assert!(json_read(b"{\"a\":").is_err(), "a body that is no JSON stays a failure");
    }

    #[test]
    fn an_empty_body_reads_as_null_whatever_the_kind() {
        let reply = Reply::<Any>::parse(answer(b""), &read_request()).expect("an empty body");
        let text = Reply::<Text>::parse(answer(b""), &read_request()).expect("an empty body");

        assert_eq!(reply.value(), &Value::Null, "there was nothing to read");
        assert_eq!(reply.read(), Some(&Value::Null), "anything reads a null");
        assert_eq!(text.read(), None, "a null is no text");
    }

    fn answer(body: &[u8]) -> Response {
        assert!(body.len() <= 64, "a fixture answer is short");

        answer_of(body)
    }

    fn answer_of(body: &[u8]) -> Response {
        let response = Response {
            body: body.to_vec(),
            is_near_limit: false,
            location: None,
            rate_limit_reason: None,
            retry_after_seconds: None,
            status: 200,
            streamed_count: 0,
        };

        assert_eq!(response.status, 200, "a fixture answer succeeds");

        response
    }
}
