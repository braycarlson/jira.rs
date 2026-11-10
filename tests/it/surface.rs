use std::io::Cursor;

use jira::assets::{AQLFindObjects, aql_find_objects};
use jira::automation::SearchManualRulesWithGet;
use jira::platform::shape::{IssueBean, Project, UserDetails};
use jira::platform::{
    AddAttachment,
    CreateIssue,
    DeleteCustomField,
    FindUsers,
    GetAttachmentContent,
    GetIssue,
    GetServerInfo,
    SearchAndReconsileIssuesUsingJQL,
    SearchProjects,
    get_issue,
    search_and_reconsile_issues_using_jql,
    search_projects,
};
use jira::software::RankIssues;
use jira::{
    AccountID,
    Agent,
    Any,
    AttachmentID,
    Base,
    BoardID,
    Boolean,
    Client,
    CommentID,
    Credentials,
    Error,
    Field,
    Integer,
    IssueID,
    IssueKey,
    Kind as _,
    List,
    Map,
    Method,
    Next,
    Number,
    Object,
    Operation,
    Paged,
    Paging,
    Part,
    ProjectKey,
    Request,
    Response,
    SprintID,
    Text,
    Upload,
    adf,
    limits,
    service_desk,
    software,
};
use serde_json::{Value, json};

use crate::fake::{CLOUD_ID, Scripted, answer, client, dropped, response, url_of};

const FLAGS: Field<Shape, Map<Boolean>> = Field::new("flags");
const LABELS: Field<Shape, List<Text>> = Field::new("labels");
const PARENT: Field<Shape, Object<Shape>> = Field::new("parent");
const RATIO: Field<Shape, Number> = Field::new("ratio");
const SIZE: Field<Shape, Integer> = Field::new("size");

#[derive(Debug)]
struct Custom<'call> {
    name: &'call str,
}

#[derive(Clone, Copy, Debug)]
struct Shape;

#[derive(Debug)]
struct Named<'call> {
    name: &'call str,
}

impl Operation for Named<'_> {
    const BASE: Base = Base::Jira;
    const IS_READ: bool = true;
    const METHOD: Method = Method::Get;
    const PATH: &'static str = "/rest/api/3/named/{name}";

    type Options<'options> = ();
    type Reply = Object<Shape>;

    fn request(&self, _options: &(), request: &mut Request) -> Result<(), Error> {
        assert!(!self.name.is_empty(), "a fixture names something");
        assert!(request.url().contains("{name}"), "the path holds its placeholder");

        request.path_text("name", self.name)
    }
}

impl Operation for Custom<'_> {
    const BASE: Base = Base::Jira;
    const IS_READ: bool = true;
    const METHOD: Method = Method::Get;
    const PATH: &'static str = "/rest/api/3/custom/{name}";

    type Options<'options> = ();
    type Reply = Any;

    fn request(&self, _options: &(), request: &mut Request) -> Result<(), Error> {
        assert!(!self.name.is_empty(), "a fixture names something");
        assert!(request.url().contains("{name}"), "the path holds its placeholder");

        request.path_text("name", self.name)
    }
}

#[test]
fn a_binary_operation_streams_into_its_sink_after_a_retry() {
    let jira = client(vec![dropped(), answer(200, &json!("bytes"))]);
    let mut sink = Cursor::new(Vec::new());
    let written = jira.download(&GetAttachmentContent { id: "10" }, &mut sink).expect("a download");
    let seen = jira.network().seen();

    assert_eq!(written, 7, "the seven bytes of the quoted word");
    assert_eq!(sink.get_ref().as_slice(), b"\"bytes\"", "the second answer landed");
    assert_eq!(seen[0].accept, "*/*", "a download takes whatever Jira sends");
    assert_eq!(seen.len(), 2, "a read repeats");
}

#[test]
fn a_custom_operation_reaches_an_endpoint_no_specification_names() {
    let jira = client(vec![answer(200, &json!({"ok": true}))]);
    let reply = jira.send(&Custom { name: "a/b" }).expect("a reply");

    assert_eq!(reply.value()["ok"], true, "the raw value reads");
    assert_eq!(jira.network().seen()[0].url, url_of("/rest/api/3/custom/a%2Fb"), "escaped");
}

#[test]
fn a_custom_shape_reads_every_kind_a_member_can_hold() {
    let body = json!({
        "flags": {"ready": true, "text": "no"},
        "labels": ["a", 7_i64, "b"],
        "parent": {"size": 3_i64},
        "ratio": 0.5_f64,
        "size": 9_i64,
    });

    let jira = client(vec![answer(200, &body)]);
    let reply = jira.send(&Named { name: "a" }).expect("a reply");
    let record = reply.read().expect("an object");
    let flags = record.member(FLAGS).expect("a map");
    let labels: Vec<&str> = record.member(LABELS).expect("a list").collect();
    let entries: Vec<(&str, bool)> = flags.entries().collect();
    let parent = record.member(PARENT).expect("a nested record");

    assert_eq!(flags.member("ready"), Some(true), "a map member reads by key");
    assert_eq!(flags.count(), 2, "both members count");
    assert_eq!(entries, [("ready", true)], "an entry of another kind is passed over");
    assert_eq!(labels, ["a", "b"], "an item of another kind is passed over");
    assert_eq!(parent.member(SIZE), Some(3), "a record nests in its own schema");
    assert_eq!(record.member(RATIO), Some(0.5_f64), "a number");
    assert_eq!(<Any as jira::Kind>::read(&json!(1_i64)), Some(&json!(1_i64)), "anything at all");
}

#[test]
fn a_download_at_its_guard_lands_and_one_byte_past_it_is_refused() {
    let mut exact = response(200, &Value::Null);
    let mut over = response(200, &Value::Null);

    exact.streamed_count = limits::ATTACHMENT_BYTES_MAX;
    over.streamed_count = limits::ATTACHMENT_BYTES_MAX.strict_add(1);

    let jira = client(vec![Scripted::Answered(exact), Scripted::Answered(over)]);
    let operation = GetAttachmentContent { id: "1" };
    let written = jira.download(&operation, &mut Cursor::new(Vec::new())).expect("at the guard");
    let failure = jira.download(&operation, &mut Cursor::new(Vec::new())).expect_err("past it");

    assert_eq!(written, limits::ATTACHMENT_BYTES_MAX, "the guard itself is allowed");
    assert!(matches!(failure, Error::Reply(_)), "one byte past the guard is refused");
}

#[test]
fn a_download_keeps_the_position_the_sink_was_handed_at() {
    let jira = client(vec![dropped(), answer(200, &json!("new"))]);
    let mut sink = Cursor::new(b"header:".to_vec());

    sink.set_position(7);

    let written = jira.download(&GetAttachmentContent { id: "10" }, &mut sink).expect("a download");

    assert_eq!(written, 5, "the five bytes of the quoted word");
    assert_eq!(sink.get_ref().as_slice(), b"header:\"new\"", "what came before stays");
}

#[test]
fn a_failed_status_is_a_refusal_that_keeps_the_status() {
    let jira = client(vec![answer(404, &json!({"errorMessages": ["gone"]}))]);
    let failure = jira.send(&GetIssue { issue_id_or_key: "A-1" }).expect_err("a refusal");

    assert_eq!(failure.status(), Some(404), "the status");
    assert!(failure.message().contains("errorMessages"), "the body is excerpted");
    assert!(failure.to_string().contains("/rest/api/3/issue/A-1"), "the request is named");
}

#[test]
fn a_filtered_user_search_answers_one_window() {
    let jira = client(vec![answer(200, &json!([{"accountId": "a"}]))]);
    let reply = jira.send(&FindUsers).expect("a single window");

    assert_eq!(reply.read().map_or(0, Iterator::count), 1, "the window is read by hand");
    assert_eq!(jira.network().seen().len(), 1, "one window");
}

#[test]
fn a_landed_write_with_an_unreadable_body_is_still_a_success() {
    let mut landed = response(201, &Value::Null);

    landed.body = b"<html>created</html>".to_vec();

    let jira = client(vec![Scripted::Answered(landed)]);
    let body = json!({"fields": {"summary": "words"}});
    let reply = jira.send(&CreateIssue { body: &body }).expect("the write landed");

    assert_eq!(reply.value(), &json!("<html>created</html>"), "what came back is kept as text");
    assert!(reply.read().is_none(), "and it does not read as the shape it should have been");
    assert_eq!(jira.network().seen().len(), 1, "and nothing was repeated");
}

#[test]
fn a_link_walk_reads_the_next_cursor_out_of_the_address_jira_returns() {
    let first = json!({
        "data": [{"id": "1"}],
        "links": {"next": "https://api.atlassian.com/rules?limit=1&cursor=c%2F2"},
    });

    let second = json!({"data": [{"id": "2"}], "links": {"next": null}});
    let jira = client(vec![answer(200, &first), answer(200, &second)]);
    let operation = SearchManualRulesWithGet { cursor: "start" };
    let pages: Vec<_> = jira.walk(&operation).collect();
    let seen = jira.network().seen();

    assert_eq!(pages.len(), 2, "both pages were walked");
    assert!(seen[0].url.ends_with("cursor=start"), "the first page uses what the caller gave");
    assert!(seen[1].url.ends_with("cursor=c%2F2"), "the second replaces it, escaped once");
    assert_eq!(seen[1].url.matches("cursor=").count(), 1, "the term is replaced, not repeated");
}

#[test]
fn a_link_walk_stops_with_a_fault_when_the_next_page_outgrows_a_url() {
    let limit = usize::try_from(limits::URL_BYTES_MAX).expect("a small limit");
    let long = "x".repeat(limit.strict_sub(2));
    let first = json!({"data": [{"id": "1"}], "links": {"next": format!("https://x/r?c={long}")}});
    let jira = client(vec![answer(200, &first)]);
    let operation = SearchManualRulesWithGet { cursor: "start" };
    let pages: Vec<_> = jira.walk(&operation).collect();

    assert!(pages[0].is_ok(), "the page in hand is delivered");
    assert!(matches!(pages[1], Err(Error::Reply(_))), "the fault is Jira's, not the caller's");
    assert_eq!(pages.len(), 2, "and the walk stops there");
    assert_eq!(jira.network().seen().len(), 1, "with nothing sent");
}

#[test]
fn a_link_walk_with_bytes_a_url_cannot_carry_follows_them_escaped() {
    let first = json!({"data": [{"id": "1"}], "links": {"next": "https://x/r?cursor={\"a b\"}"}});
    let second = json!({"data": [{"id": "2"}], "links": {"next": null}});
    let jira = client(vec![answer(200, &first), answer(200, &second)]);
    let operation = SearchManualRulesWithGet { cursor: "start" };
    let pages: Vec<_> = jira.walk(&operation).collect();
    let seen = jira.network().seen();

    assert!(pages.iter().all(Result::is_ok), "a brace in a query is no unfilled placeholder");
    assert!(seen[1].url.ends_with("cursor=%7B%22a%20b%22%7D"), "escaped: {}", seen[1].url);
}

#[test]
fn a_multipart_body_reaches_the_wire_with_its_xsrf_header() {
    let jira = client(vec![answer(200, &json!([{"id": "10"}]))]);

    let parts = [Part {
        bytes: b"hello",
        content_type: "text/plain",
        file_name: Some("a.txt"),
        name: "file",
    }];

    let reply = jira.send(&AddAttachment { body: &parts, issue_id_or_key: "A-1" }).expect("sent");
    let seen = jira.network().seen();
    let body = String::from_utf8(seen[0].body.clone()).expect("plain text");

    assert_eq!(seen[0].method, Method::Post, "the method");
    assert!(body.contains("filename=\"a.txt\""), "the part is named");
    assert!(body.contains("hello"), "the part is sent");
    assert!(seen[0].headers.contains(&header_xsrf()), "the header the endpoint demands");
    assert_eq!(reply.read().expect("a list").count(), 1, "the reply lists the attachment");
}

#[test]
fn a_near_limit_answer_pauses_after_it_succeeds() {
    let mut paced = response(200, &json!({"key": "A-1"}));

    paced.is_near_limit = true;

    let jira = client(vec![Scripted::Answered(paced)]);
    let _reply = jira.send(&GetIssue { issue_id_or_key: "A-1" }).expect("a reply");
    let waits = jira.network().waits();

    assert_eq!(waits.len(), 1, "one pause");
    assert_eq!(waits[0].reason, "near the limit", "the pause says why");
    assert_eq!(waits[0].millis, 250, "the documented pause");
    assert!(waits[0].note.is_none(), "a pause is no failure");
}

#[test]
fn a_nested_token_walk_reads_its_pointers() {
    let first = json!({"projects": {"nextPageToken": "t2", "values": [{"id": "1"}]}});
    let second = json!({"projects": {"values": [{"id": "2"}]}});
    let jira = client(vec![answer(200, &first), answer(200, &second)]);
    let operation = jira::platform::GetProjectUsagesForWorkflow { workflow_id: "w" };
    let mut count = 0_u64;

    for page in jira.walk(&operation) {
        count = count.strict_add(page.expect("a page").count());
    }

    assert_eq!(count, 2, "both pages were walked");
    assert!(jira.network().seen()[1].url.contains("nextPageToken=t2"), "the token was sent");
}

#[test]
fn a_page_number_walk_counts_up_from_one_and_stops_at_the_total() {
    let first = json!({"objectEntries": [{"id": "1"}], "totalFilterCount": 2_i64});
    let second = json!({"objectEntries": [{"id": "2"}], "totalFilterCount": 2_i64});
    let jira = client(vec![answer(200, &first), answer(200, &second)]);
    let operation = AQLFindObjects { workspace_id: "w" };
    let pages: Vec<_> = jira.walk(&operation).collect();
    let seen = jira.network().seen();

    assert_eq!(pages.len(), 2, "two pages");
    assert!(!seen[0].url.contains("page="), "the first page names no number");
    assert!(seen[1].url.ends_with("page=2"), "the second does");
    assert!(seen[0].url.contains("/jsm/assets/workspace/w/v1/aql/objects"), "the workspace path");
}

#[test]
fn a_pause_that_fails_cannot_lose_a_landed_write() {
    let mut paced = response(201, &json!({"key": "A-2"}));

    paced.is_near_limit = true;

    let jira = client(vec![Scripted::Answered(paced)]);

    jira.network().wait_refuse();

    let body = json!({"fields": {"summary": "words"}});
    let reply = jira.send(&CreateIssue { body: &body }).expect("the write landed");

    assert_eq!(reply.value()["key"], "A-2", "the reply survives the failed pause");
    assert_eq!(jira.network().waits().len(), 1, "the pause was attempted");
}

#[test]
fn a_read_retries_a_server_failure_and_says_why_it_waited() {
    let jira = client(vec![answer(503, &Value::Null), answer(200, &json!({"key": "A-1"}))]);
    let reply = jira.send(&GetIssue { issue_id_or_key: "A-1" }).expect("the second attempt");
    let waits = jira.network().waits();

    assert_eq!(reply.read().and_then(|issue| issue.member(IssueBean::KEY)), Some("A-1"), "read");
    assert_eq!(waits.len(), 1, "one retry");
    assert_eq!(waits[0].reason, "server error", "the wait says why");
    assert!(waits[0].millis >= 3_500, "the first delay is jittered around five seconds");
    assert!(waits[0].note.as_deref().is_some_and(|note| note.contains("retry 1 of 4")), "noted");
}

#[test]
fn a_read_that_keeps_failing_gives_up_after_four_retries_and_names_the_request() {
    let attempts = vec![dropped(), dropped(), dropped(), dropped(), dropped()];
    let jira = client(attempts);
    let failure = jira.send(&GetIssue { issue_id_or_key: "A-1" }).expect_err("a lost network");

    assert!(matches!(failure, Error::Transport(_)), "a read is safe to call a transport failure");
    assert!(failure.message().contains("/rest/api/3/issue/A-1"), "the request is named");
    assert_eq!(jira.network().seen().len(), 5, "the first attempt and four retries");
    assert_eq!(jira.network().answers_left(), 0, "the script ran out exactly");
}

#[test]
fn a_record_reads_typed_members_and_keeps_the_rest_of_the_value() {
    let body = json!({
        "archived": false,
        "key": "ABC",
        "lead": {"displayName": "Ada"},
        "customThing": 7_i64,
    });

    let jira = client(vec![answer(200, &body)]);
    let operation = jira::platform::GetProject { project_id_or_key: "ABC" };
    let reply = jira.send(&operation).expect("a project");
    let project = reply.read().expect("an object");
    let lead = project.member(Project::LEAD).expect("a lead");

    assert_eq!(project.member(Project::KEY), Some("ABC"), "a text");
    assert_eq!(project.member(Project::ARCHIVED), Some(false), "a boolean");
    assert!(lead.value().is_object(), "a nested record");
    assert_eq!(project.value()["customThing"], 7_i64, "what the schema never named stays");
}

#[test]
fn a_redirect_on_a_write_is_the_task_it_started() {
    let mut accepted = response(303, &Value::Null);

    accepted.location = Some("https://site/rest/api/3/task/10".to_owned());

    let jira = client(vec![Scripted::Answered(accepted)]);
    let reply = jira.send(&DeleteCustomField { id: "customfield_1" }).expect("an accepted task");

    assert_eq!(reply.location(), Some("https://site/rest/api/3/task/10"), "the task to poll");
    assert_eq!(jira.network().seen()[0].method, Method::Delete, "the method");
}

#[test]
fn a_repeated_token_stops_the_walk_after_the_page_it_arrived_with() {
    let first = json!({"issues": [{"id": "1"}], "nextPageToken": "t"});
    let second = json!({"issues": [{"id": "2"}], "nextPageToken": "t"});
    let jira = client(vec![answer(200, &first), answer(200, &second)]);
    let pages: Vec<_> = jira.walk(&SearchAndReconsileIssuesUsingJQL).collect();

    assert_eq!(pages.len(), 3, "two pages, then the refusal");
    assert!(pages[1].is_ok(), "the second page still arrives");
    assert!(matches!(pages[2], Err(Error::Reply(_))), "and the loop is named rather than run");
}

#[test]
fn a_reply_names_the_status_that_tells_a_partial_success_from_a_whole_one() {
    let partial = json!({"entries": [{"issueId": 1_i64, "status": 400_i64}]});
    let jira = client(vec![answer(207, &partial), answer(204, &Value::Null)]);
    let body = json!({"issues": ["A-1"], "rankAfterIssue": "A-2"});
    let reply = jira.send(&RankIssues { body: &body }).expect("a success with a failure inside");
    let whole = jira.send(&RankIssues { body: &body }).expect("a whole success");

    assert_eq!(reply.status(), 207, "the part that failed is in the body");
    assert_eq!(reply.value(), &partial, "and the body is kept though no schema promised one");
    assert_eq!(reply.read(), Some(()), "the type still promises nothing");
    assert_eq!(whole.status(), 204, "a whole success says so");
}

#[test]
fn a_required_parameter_is_a_field_and_an_option_is_a_second_literal() {
    let jira = client(vec![answer(200, &json!({"key": "A-1"})), answer(200, &json!({}))]);
    let plain = GetIssue { issue_id_or_key: "A-1" };

    let options = get_issue::Options {
        expand: Some("names"),
        fail_fast: None,
        fields: Some(&["summary", "status"]),
        fields_by_keys: Some(true),
        properties: None,
        update_history: None,
    };

    let _bare = jira.send(&plain).expect("a reply");
    let _configured = jira.send_with(&plain, &options).expect("a reply");
    let seen = jira.network().seen();

    assert_eq!(seen[0].url, url_of("/rest/api/3/issue/A-1"), "no option, no query");
    assert!(seen[0].authorization.starts_with("Basic "), "every request carries the credential");

    assert_eq!(
        seen[1].url,
        url_of("/rest/api/3/issue/A-1?expand=names&fields=summary&fields=status&fieldsByKeys=true"),
        "each option under its wire name",
    );
}

#[test]
fn a_retry_after_at_its_guard_is_honored_and_one_second_more_ends_the_retries() {
    let mut honored = response(429, &Value::Null);
    let mut refused = response(429, &Value::Null);

    honored.retry_after_seconds = Some(900);
    refused.retry_after_seconds = Some(901);

    let jira = client(vec![Scripted::Answered(honored), answer(200, &json!({}))]);

    assert!(jira.send(&GetServerInfo).is_ok(), "a wait of 15 minutes is honored");
    assert!(jira.network().waits()[0].millis >= 900_000, "the wait is the one Jira asked for");

    let cut_short = client(vec![Scripted::Answered(refused)]);
    let failure = cut_short.send(&GetServerInfo).expect_err("a wait past the guard ends retries");

    assert_eq!(failure.status(), Some(429), "the refusal comes back");
    assert!(cut_short.network().waits().is_empty(), "nothing was waited for");
}

#[test]
fn a_retry_after_is_a_floor_the_jitter_never_undercuts() {
    for _attempt_index in 0..32_u32 {
        let mut throttled = response(429, &Value::Null);

        throttled.retry_after_seconds = Some(30);

        let jira = client(vec![Scripted::Answered(throttled), answer(200, &json!({}))]);

        assert!(jira.send(&GetServerInfo).is_ok(), "the second attempt lands");
        assert!(jira.network().waits()[0].millis >= 30_000, "the jitter stays above the floor");
    }
}

#[test]
fn a_rewritten_authorization_never_reaches_a_diagnostic() {
    let token = "token123";
    let encoded = "dXNlckBleGFtcGxlLmNvbTp0b2tlbjEyMw==";
    let credentials = Credentials::Token { cloud_id: CLOUD_ID, email: "user@example.com", token };
    let jira = Client::new(Agent::new(), &credentials).expect("a client");
    let faked = client(vec![answer(404, &json!({"errorMessages": ["gone"]}))]);
    let refused = faked.send(&GetServerInfo).expect_err("a refusal");
    let walk = format!("{:?}", jira.walk(&SearchProjects));

    assert_eq!(
        faked.network().seen()[0].authorization,
        format!("Basic {encoded}"),
        "the wire form",
    );

    assert!(format!("{jira:?}").contains(CLOUD_ID), "the client names its site");

    for printed in [
        format!("{credentials:?}"),
        format!("{jira:?}"),
        format!("{faked:?}"),
        format!("{refused:?}"),
        refused.to_string(),
        walk,
    ] {
        assert!(!printed.contains(token), "the token stays out: {printed}");
        assert!(!printed.contains(encoded), "and so does the header it became: {printed}");
    }
}

#[test]
fn a_throttled_write_is_retried_because_it_never_landed() {
    let mut throttled = response(429, &Value::Null);

    throttled.retry_after_seconds = Some(1);
    throttled.rate_limit_reason = Some("jira-burst-based".to_owned());

    let jira = client(vec![Scripted::Answered(throttled), answer(201, &json!({"key": "A-2"}))]);
    let body = json!({"fields": {"summary": "words"}});
    let reply = jira.send(&CreateIssue { body: &body }).expect("the second attempt");
    let waits = jira.network().waits();

    assert_eq!(reply.value()["key"], "A-2", "created");
    assert_eq!(waits[0].reason, "rate limited", "the wait says why");
    assert!(waits[0].millis >= 1_000, "the delay Jira asked for is a floor");

    assert!(
        waits[0].note.as_deref().is_some_and(|note| note.contains("jira-burst-based")),
        "the reason Jira gave is named",
    );
}

#[test]
fn a_token_walk_follows_its_tokens_to_the_last_page() {
    let first = json!({"issues": [{"id": "1"}], "nextPageToken": "t2"});
    let second = json!({"isLast": true, "issues": [{"id": "2"}]});
    let jira = client(vec![answer(200, &first), answer(200, &second)]);

    let options = search_and_reconsile_issues_using_jql::Options {
        expand: None,
        fail_fast: None,
        fields: None,
        fields_by_keys: None,
        include_archived_projects: None,
        jql: Some("project = A"),
        page_token_next: None,
        properties: None,
        reconcile_issues: None,
        results_max: None,
    };

    let mut ids: Vec<String> = Vec::new();

    for walked in jira.walk_with(&SearchAndReconsileIssuesUsingJQL, options) {
        let found = walked.expect("a page");
        let issues = found.items().filter_map(|issue| issue.member(IssueBean::ID));

        ids.extend(issues.map(str::to_owned));
    }

    let seen = jira.network().seen();

    assert_eq!(ids, ["1", "2"], "both pages in order");
    assert!(!seen[0].url.contains("nextPageToken"), "the first page names no token");
    assert!(seen[1].url.ends_with("nextPageToken=t2"), "the second names the token");
    assert!(seen[1].url.contains("jql=project%20%3D%20A"), "the options ride on every page");
}

#[test]
fn a_walk_given_an_empty_body_is_a_fault_of_the_reply_rather_than_a_crash() {
    let mut empty = response(200, &Value::Null);

    empty.body = Vec::new();

    let jira = client(vec![Scripted::Answered(empty)]);
    let pages: Vec<_> = jira.walk(&SearchProjects).collect();

    assert_eq!(pages.len(), 1, "one failure and then nothing");
    assert!(matches!(pages[0], Err(Error::Reply(_))), "an empty page is no page");
}

#[test]
fn a_walk_handed_tokens_it_already_used_stops_within_a_lap_of_the_circle() {
    let tokens = ["a", "b", "c", "b", "c"];
    let bodies = tokens.map(|text| json!({"issues": [{"id": "1"}], "nextPageToken": text}));
    let jira = client(bodies.iter().map(|body| answer(200, body)).collect());
    let pages: Vec<_> = jira.walk(&SearchAndReconsileIssuesUsingJQL).collect();

    assert!(pages[..pages.len().strict_sub(1)].iter().all(Result::is_ok), "each page is delivered");
    assert!(matches!(pages.last(), Some(Err(Error::Reply(_)))), "and the circle is named");
    assert_eq!(pages.len(), 6, "the five pages and the failure");
    assert_eq!(jira.network().answers_left(), 0, "nothing more was asked");
}

#[test]
fn a_walk_keeps_a_page_that_holds_one_document_nested_past_the_parser() {
    let depth = 2_000;
    let deep = format!("{}\"bottom\"{}", "[".repeat(depth), "]".repeat(depth));
    let first = format!("{{\"key\":\"A\",\"deep\":{deep}}}");
    let raw = format!("{{\"isLast\":true,\"values\":[{first},{{\"key\":\"B\"}}]}}");
    let mut page = response(200, &Value::Null);

    page.body = raw.into_bytes();

    let jira = client(vec![Scripted::Answered(page)]);
    let pages: Vec<_> = jira.walk(&SearchProjects).collect();
    let found = pages[0].as_ref().expect("the page survives one deep document");

    assert_eq!(found.count(), 2, "both projects arrive");
    assert_eq!(found.value()["values"][1]["key"], "B", "and the one after the cut is whole");
}

#[test]
fn a_walk_may_start_at_its_cursor_budget_and_never_past_it() {
    let at = i64::try_from(limits::CURSOR_MAX).expect("the budget fits a cursor");
    let jira = client(vec![answer(200, &json!({"values": [], "isLast": true}))]);
    let pages: Vec<_> = jira.walk_with(&SearchProjects, projects_from(at)).collect();

    assert_eq!(pages.len(), 1, "a walk may begin at its budget");
    assert!(pages[0].is_ok(), "and reads the page it finds");

    let refusing = client(vec![answer(200, &json!({"values": [], "isLast": true}))]);
    let options = projects_from(at.strict_add(1));
    let past: Vec<_> = refusing.walk_with(&SearchProjects, options).collect();

    assert!(matches!(past[0], Err(Error::Input(_))), "the option was the caller's");
    assert!(refusing.network().seen().is_empty(), "and nothing is asked of Jira");
}

#[test]
fn a_walk_of_a_list_that_moved_under_it_repeats_a_page_once_and_carries_on() {
    let open = ["A", "A", "B"].map(|key| json!({"isLast": false, "values": [{"key": key}]}));
    let last = json!({"isLast": true, "values": [{"key": "C"}]});
    let bodies = open.iter().chain([&last]);
    let jira = client(bodies.map(|body| answer(200, body)).collect());
    let pages: Vec<_> = jira.walk(&SearchProjects).collect();

    assert_eq!(pages.len(), 4, "a burst of inserts shifted one page onto the next");
    assert!(pages.iter().all(Result::is_ok), "and that is no reason to give up");
}

#[test]
fn a_walk_of_a_server_that_ignores_its_offset_stops_rather_than_repeating_forever() {
    let stuck = json!({"isLast": false, "values": [{"key": "A"}]});
    let jira = client(vec![answer(200, &stuck), answer(200, &stuck), answer(200, &stuck)]);
    let pages: Vec<_> = jira.walk(&SearchProjects).collect();
    let failure = pages.last().and_then(|page| page.as_ref().err()).expect("the walk failed");

    assert_eq!(pages.len(), 4, "three pages and the failure");
    assert!(matches!(failure, Error::Reply(_)), "the fault is the reply's");
    assert!(failure.message().contains("/rest/api/3/project/search"), "the failure names the walk");
    assert_eq!(jira.network().answers_left(), 0, "nothing more was asked");
}

#[test]
fn a_walk_of_numbered_pages_from_the_start_says_when_it_ended_short() {
    let first = json!({"objectEntries": [{"id": 1_i64}], "totalFilterCount": 3_i64});
    let empty = json!({"objectEntries": [], "totalFilterCount": 3_i64});
    let jira = client(vec![answer(200, &first), answer(200, &empty)]);
    let operation = AQLFindObjects { workspace_id: "workspace" };
    let pages: Vec<_> = jira.walk(&operation).collect();

    assert_eq!(pages.len(), 2, "the page and the empty one after it");
    assert!(short_at_end(&pages), "one of three is short of the total");
}

#[test]
fn a_walk_of_numbered_pages_started_part_way_cannot_say_it_ended_short() {
    let page = json!({"objectEntries": [{"id": 1_i64}], "totalFilterCount": 203_i64});
    let empty = json!({"objectEntries": [], "totalFilterCount": 203_i64});
    let jira = client(vec![answer(200, &page), answer(200, &empty)]);

    let options = aql_find_objects::Options {
        include_attributes: None,
        include_attributes_deep: None,
        include_extended_information: None,
        include_type_attributes: None,
        page: Some(5),
        ql_query: None,
        result_per_page: Some(50),
    };

    let operation = AQLFindObjects { workspace_id: "workspace" };
    let pages: Vec<_> = jira.walk_with(&operation, options).collect();

    assert_eq!(pages.len(), 2, "the last page and the empty one after it");
    assert!(!short_at_end(&pages), "its position is only an estimate, so it makes no claim");
}

#[test]
fn a_walk_of_numbered_pages_started_part_way_stops_at_its_total() {
    let bodies = [[3_i64, 4], [5, 6]].map(
        |ids| json!({"objectEntries": [{"id": ids[0]}, {"id": ids[1]}], "totalFilterCount": 6_i64}),
    );

    let jira = client(bodies.iter().map(|body| answer(200, body)).collect());

    let options = aql_find_objects::Options {
        include_attributes: None,
        include_attributes_deep: None,
        include_extended_information: None,
        include_type_attributes: None,
        page: Some(2),
        ql_query: None,
        result_per_page: Some(2),
    };

    let operation = AQLFindObjects { workspace_id: "workspace" };
    let pages: Vec<_> = jira.walk_with(&operation, options).collect();
    let seen = jira.network().seen();

    assert_eq!(pages.len(), 2, "pages two and three of three");
    assert!(pages.iter().all(Result::is_ok), "and each of them read");
    assert!(seen[1].url.contains("page=3"), "the second counts from the first: {}", seen[1].url);
    assert_eq!(jira.network().answers_left(), 0, "page four is never asked for");
    assert!(!short_at_end(&pages), "and a walk that began part way never claims to be short");
}

#[test]
fn a_walk_of_offsets_started_part_way_still_says_when_it_ended_short() {
    let page = json!({"total": 150_i64, "values": [{"key": "A"}]});
    let empty = json!({"total": 150_i64, "values": []});
    let jira = client(vec![answer(200, &page), answer(200, &empty)]);
    let pages: Vec<_> = jira.walk_with(&SearchProjects, projects_from(100)).collect();

    assert_eq!(pages.len(), 2, "the page and the empty one after it");
    assert!(short_at_end(&pages), "an offset is exact, so 101 of 150 is known to be short");
}

#[test]
fn a_walk_prints_where_it_stands() {
    let jira = client(vec![answer(200, &json!({"values": []}))]);
    let walk = jira.walk(&SearchProjects);
    let printed = format!("{walk:?}");

    assert!(printed.starts_with("Walk"), "the type is named: {printed}");
    assert!(printed.contains("page_count_remaining"), "the budget shows: {printed}");
    assert!(!printed.contains("token123"), "no credential prints: {printed}");
    assert!(!printed.contains("Basic"), "not even as a header: {printed}");
}

#[test]
fn a_walk_starts_where_the_options_say_and_moves_forward_from_there() {
    let first = json!({"isLast": false, "total": 150_i64, "values": [{"key": "A"}, {"key": "B"}]});
    let second = json!({"isLast": true, "total": 150_i64, "values": [{"key": "C"}]});
    let jira = client(vec![answer(200, &first), answer(200, &second)]);

    let options = search_projects::Options {
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
        start_at: Some(100),
        status: None,
        type_key: None,
    };

    let pages: Vec<_> = jira.walk_with(&SearchProjects, options).collect();
    let seen = jira.network().seen();

    assert_eq!(pages.len(), 2, "both pages");
    assert!(seen[0].url.ends_with("startAt=100"), "the first page starts where it was told");
    assert!(seen[1].url.ends_with("startAt=102"), "the second follows it, never before it");
}

#[test]
fn a_walk_stops_at_an_empty_page_that_says_more_follow() {
    let first = json!({"isLast": false, "total": 120_i64, "values": [{"key": "A"}]});
    let empty = json!({"isLast": false, "total": 120_i64, "values": []});
    let jira = client(vec![answer(200, &first), answer(200, &empty)]);
    let pages: Vec<_> = jira.walk(&SearchProjects).collect();

    assert!(pages[0].is_ok(), "the first page arrived");
    assert!(matches!(pages[1], Err(Error::Reply(_))), "the rest is not quietly dropped");
    assert_eq!(pages.len(), 2, "and the walk ends there");
}

#[test]
fn a_walk_stops_at_the_first_failure() {
    let first = json!({"isLast": false, "values": [{"key": "A"}]});
    let jira = client(vec![answer(200, &first), answer(404, &Value::Null)]);
    let pages: Vec<_> = jira.walk(&SearchProjects).collect();

    assert_eq!(pages.len(), 2, "the page, then the failure, then nothing");
    assert!(pages[0].is_ok(), "the first page arrived");

    assert_eq!(pages[1].as_ref().err().and_then(Error::status), Some(404), "the refusal");
}

#[test]
fn a_walk_that_ends_short_of_its_total_says_so_on_its_last_page() {
    let first = json!({"total": 120_i64, "values": [{"key": "A"}]});
    let second = json!({"total": 120_i64, "values": []});
    let jira = client(vec![answer(200, &first), answer(200, &second)]);
    let pages: Vec<_> = jira.walk(&SearchProjects).map(|page| page.expect("a page")).collect();

    assert!(!pages[0].is_short(), "a page with more to come is not short");
    assert!(pages[1].is_short(), "the last page tells the caller the walk saw 1 of 120");
    assert_eq!(pages.len(), 2, "and no failure stopped it");
}

#[test]
fn a_write_is_not_repeated_after_a_failure_that_may_have_landed() {
    let lost = client(vec![dropped()]);
    let failed = client(vec![answer(503, &Value::Null)]);
    let body = json!({"fields": {"summary": "words"}});
    let operation = CreateIssue { body: &body };
    let indefinite = lost.send(&operation).expect_err("a lost write");
    let refused = failed.send(&operation).expect_err("a failed write");

    assert!(matches!(indefinite, Error::Indefinite(_)), "the write may have landed");
    assert!(indefinite.message().contains("may have landed"), "and the message says so");
    assert!(matches!(refused, Error::Indefinite(_)), "a gateway may have let the write through");
    assert!(refused.message().contains("503"), "and the status stays in the message");
    assert_eq!(lost.network().seen().len(), 1, "one attempt");
    assert_eq!(failed.network().seen().len(), 1, "one attempt");
}

#[test]
fn a_write_sends_its_json_and_reads_its_reply() {
    let jira = client(vec![answer(201, &json!({"id": "10", "key": "A-2"}))]);
    let body = json!({"fields": {"summary": "words"}});
    let reply = jira.send(&CreateIssue { body: &body }).expect("created");
    let seen = jira.network().seen();

    assert_eq!(seen[0].method, Method::Post, "the method");
    assert_eq!(seen[0].content_type.as_deref(), Some("application/json"), "the type");
    assert_eq!(seen[0].body, serde_json::to_vec(&body).expect("JSON"), "the bytes");
    assert!(!seen[0].is_read, "a create is a write");
    assert_eq!(reply.value()["key"], "A-2", "the reply");
}

#[test]
fn an_empty_page_with_a_token_keeps_the_walk_going() {
    let filtered = json!({"isLast": false, "issues": [], "nextPageToken": "t2"});
    let last = json!({"isLast": true, "issues": [{"id": "1"}]});
    let jira = client(vec![answer(200, &filtered), answer(200, &last)]);
    let pages: Vec<_> = jira.walk(&SearchAndReconsileIssuesUsingJQL).collect();

    assert_eq!(pages.len(), 2, "the empty page does not end the walk");
    assert_eq!(pages[1].as_ref().map_or(0, jira::Page::count), 1, "the last page arrives");
}

#[test]
fn an_offset_walk_moves_past_what_it_has_read() {
    let first = json!({"isLast": false, "total": 3_i64, "values": [{"key": "A"}, {"key": "B"}]});
    let second = json!({"isLast": true, "total": 3_i64, "values": [{"key": "C"}]});
    let jira = client(vec![answer(200, &first), answer(200, &second)]);
    let mut keys: Vec<String> = Vec::new();

    for walked in jira.walk(&SearchProjects) {
        let found = walked.expect("a page");
        let projects = found.items().filter_map(|project| project.member(Project::KEY));

        keys.extend(projects.map(str::to_owned));
    }

    let seen = jira.network().seen();

    assert_eq!(keys, ["A", "B", "C"], "every project in order");
    assert!(!seen[0].url.contains("startAt"), "the first page names no cursor");
    assert!(seen[1].url.ends_with("startAt=2"), "the second starts after the first");
}

#[test]
fn credentials_that_could_not_reach_a_site_are_refused_at_construction() {
    let hostile = Credentials::Bearer { cloud_id: "tenant/../other", token: "x" };
    let empty = Credentials::Bearer { cloud_id: "site", token: "" };

    assert!(matches!(Client::new(Agent::new(), &hostile), Err(Error::Input(_))), "a bad site");
    assert!(matches!(Client::new(Agent::new(), &empty), Err(Error::Input(_))), "no token");
}

#[test]
fn each_product_reaches_its_own_base() {
    let jira = client(vec![answer(200, &json!({"data": []})), answer(200, &json!({}))]);
    let rules = SearchManualRulesWithGet { cursor: "" };
    let _rules = jira.send(&rules).expect("a reply");
    let _details = jira.send(&service_desk::GetInfo).expect("a reply");
    let seen = jira.network().seen();
    let automation_base = format!("https://api.atlassian.com/automation/public/jira/{CLOUD_ID}");

    assert!(
        seen[0].url.starts_with(&format!("{automation_base}/rest/v1/rule/manual/search")),
        "the automation gateway",
    );

    assert_eq!(seen[1].url, url_of("/rest/servicedeskapi/info"), "the service desk API");
}

fn header_xsrf() -> (String, String) {
    let header = ("X-Atlassian-Token".to_owned(), "no-check".to_owned());

    assert!(!header.0.is_empty(), "the header is named");
    assert!(!header.1.is_empty(), "the header has a value");

    header
}

fn projects_from(start_at: i64) -> search_projects::Options<'static> {
    assert!(start_at >= 0, "a walk starts at or after the beginning");

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

fn short_at_end<Item>(pages: &[Result<jira::Page<Item>, Error>]) -> bool
where
    Item: jira::Kind,
{
    assert!(!pages.is_empty(), "a walk read at least one page");

    pages.last().and_then(|page| page.as_ref().ok()).is_some_and(jira::Page::is_short)
}

#[test]
fn the_paged_trait_names_how_each_operation_walks() {
    let projects = <SearchProjects as Paged>::PAGING;
    let issues = <SearchAndReconsileIssuesUsingJQL as Paged>::PAGING;
    let boards = <software::GetAllBoards as Paged>::PAGING;

    assert_eq!(projects, Paging { cursor: "startAt", items: Some("/values"), next: Next::Offset });
    assert_eq!(issues.next, Next::Token("/nextPageToken"), "the enhanced search");
    assert_eq!(issues.items, Some("/issues"), "the enhanced search");
    assert_eq!(boards.items, Some("/values"), "the agile API");
}

#[test]
fn the_response_a_custom_transport_builds_is_plain_data() {
    let built = Response {
        body: b"{}".to_vec(),
        is_near_limit: false,
        location: None,
        rate_limit_reason: None,
        retry_after_seconds: Some(5),
        status: 200,
        streamed_count: 0,
    };

    let jira = client(vec![Scripted::Answered(built)]);

    assert_eq!(jira.send(&SearchProjects).expect("a reply").value(), &json!({}), "the body");
    assert_eq!(jira.network().waits().len(), 0, "a success never waits");
}

#[test]
fn the_rich_text_flattener_reads_a_document_as_prose() {
    let document = json!({
        "type": "doc",
        "content": [{"type": "paragraph", "content": [{"type": "text", "text": "hello"}]}],
    });

    assert_eq!(adf::text_read(&document), "hello", "a document");
    assert_eq!(adf::text_read(&json!("plain")), "plain", "a plain string passes through");
    assert!(adf::TRUNCATION_NOTE.contains("truncated"), "the marker names itself");
}

#[test]
fn the_typed_handles_are_valid_by_construction() {
    assert_eq!(IssueKey::parse("ABC-1").expect("a key").project_part(), "ABC", "a key");
    assert_eq!(ProjectKey::parse("ABC").expect("a key").text(), "ABC", "a project");
    assert_eq!(AccountID::parse("5b10ac").expect("an id").text(), "5b10ac", "an account");
    assert!(IssueKey::parse("a\"b").is_none(), "a quote closes a JQL term");
    assert!(ProjectKey::parse("").is_none(), "an empty key names nothing");
    assert!(AccountID::parse("a b").is_none(), "a name is not an account id");
    assert!(AttachmentID::new(0).is_none(), "zero is no id");
    assert_eq!(BoardID::new(7).expect("an id").get(), 7, "a board");
    assert_eq!(CommentID::new(7).expect("an id").get(), 7, "a comment");
    assert_eq!(IssueID::new(7).expect("an id").get(), 7, "an issue");
    assert_eq!(SprintID::new(7).expect("an id").get(), 7, "a sprint");
}

#[test]
fn the_url_limit_is_a_refusal_rather_than_a_request_jira_would_reject() {
    let jira = client(vec![answer(200, &json!({}))]);
    let long = "x".repeat(usize::try_from(limits::URL_BYTES_MAX).expect("a small limit"));
    let failure = jira.send(&GetIssue { issue_id_or_key: &long }).expect_err("too long");

    assert!(matches!(failure, Error::Input(_)), "the caller's fault");
    assert!(jira.network().seen().is_empty(), "nothing was sent");
}

#[test]
fn the_user_shape_reads_through_a_list() {
    let body = json!([{"displayName": "Ada"}, 7_i64, {"displayName": "Bo"}]);
    let users = <List<Object<UserDetails>>>::read(&body).expect("an array");

    let shown: Vec<&str> =
        users.filter_map(|user| user.member(UserDetails::DISPLAY_NAME)).collect();

    assert_eq!(shown, ["Ada", "Bo"], "a value of the wrong kind is passed over");
    assert_eq!(UserDetails::DISPLAY_NAME.name(), "displayName", "the wire name");
}

#[test]
fn upload_is_plain_data_a_caller_fills_in() {
    let upload = Upload { bytes: b"\x89PNG", content_type: "image/png" };

    assert_eq!(upload.content_type, "image/png", "the type");
    assert_eq!(upload.bytes.len(), 4, "the bytes");
}
