use std::env;
use std::fs;
use std::io::{self, Cursor, Write as _};
use std::process::ExitCode;

use jira::platform::shape::{Attachment, CreatedIssue, ServerInformation, User};
use jira::platform::{
    AddAttachment,
    AddComment,
    CreateIssue,
    DeleteIssue,
    GetAttachmentContent,
    GetComments,
    GetCurrentUser,
    GetFields,
    GetIssue,
    GetServerInfo,
    GetValidProjectKey,
    SearchAndReconsileIssuesUsingJQL,
    SearchProjects,
    get_comments,
    get_issue,
    get_valid_project_key,
    search_and_reconsile_issues_using_jql,
};
use jira::software::GetAllBoards;
use jira::{Agent, Client, Credentials, Error, Part, adf};
use serde_json::{Value, json};

const ACCOUNT_FILE: &str = "test-account.json";
const ACCOUNT_FIELDS: [&str; 5] = ["cloud_id", "email", "project_key", "site", "token"];
const ATTACHMENT_BYTES: &[u8] = b"a body written by the live check";
const COMMENT_COUNT: u64 = 3;
const ISSUE_TYPE: &str = "Task";
const SEARCH_PAGE_ITEM_COUNT: i64 = 10;
const SUMMARY: &str = "A disposable issue from the jira.rs live check";

#[derive(Debug)]
struct Account {
    project_key: String,
    settings: Value,
    site: String,
}

fn main() -> ExitCode {
    let Err(failure) = run() else {
        return ExitCode::SUCCESS;
    };

    assert!(!failure.message().is_empty(), "a failure says why");

    if writeln!(io::stderr().lock(), "live check failed: {failure}").is_err() {
        return ExitCode::from(2);
    }

    ExitCode::FAILURE
}

fn make_issue(client: &Client<Agent>, account: &Account) -> Result<String, Error> {
    assert!(!account.project_key.is_empty(), "an issue lands in a project");

    let body = json!({
        "fields": {
            "issuetype": {"name": ISSUE_TYPE},
            "project": {"key": account.project_key},
            "summary": SUMMARY,
        },
    });

    let reply = client.send(&CreateIssue { body: &body })?;
    let created = reply.read().ok_or_else(|| reply_missing("the created issue"))?;
    let key = created.member(CreatedIssue::KEY).ok_or_else(|| reply_missing("the issue key"))?;

    report(&format!("issue {key} created"))?;

    Ok(key.to_owned())
}

fn read_account() -> Result<Account, Error> {
    let path = env::args().nth(1).unwrap_or_else(|| ACCOUNT_FILE.to_owned());

    assert!(!path.is_empty(), "a live check names its credentials");

    let text = fs::read_to_string(&path)
        .map_err(|cause| Error::Input(format!("cannot read {path}: {cause}")))?;

    let settings: Value = serde_json::from_str(&text)
        .map_err(|cause| Error::Input(format!("cannot parse {path}: {cause}")))?;

    for field in ACCOUNT_FIELDS {
        if text_of(&settings, field).is_empty() {
            return Err(Error::Input(format!("{path} names no {field}")));
        }
    }

    let project_key = text_of(&settings, "project_key").to_owned();
    let site = text_of(&settings, "site").trim_end_matches('/').to_owned();

    assert!(!project_key.is_empty(), "a checked project key is present");
    assert!(!site.ends_with('/'), "a site compares without its last slash");

    Ok(Account { project_key, settings, site })
}

fn reply_missing(what: &str) -> Error {
    assert!(!what.is_empty(), "a missing reply names what is missing");

    let failure = Error::Reply(format!("the reply holds no {what}"));

    assert!(matches!(failure, Error::Reply(_)), "a missing member is a fault of the reply");

    failure
}

fn report(line: &str) -> Result<(), Error> {
    assert!(!line.is_empty(), "a report says something");
    assert!(!line.contains('\n'), "a report is one line");

    writeln!(io::stdout().lock(), "{line}")
        .map_err(|cause| Error::Input(format!("cannot write to standard output: {cause}")))
}

fn run() -> Result<(), Error> {
    let account = read_account()?;

    let credentials = Credentials::Token {
        cloud_id: text_of(&account.settings, "cloud_id"),
        email: text_of(&account.settings, "email"),
        token: text_of(&account.settings, "token"),
    };

    let client = Client::new(Agent::new(), &credentials)?;

    assert!(!account.site.is_empty(), "a live check names the site it expects");
    assert!(!account.project_key.is_empty(), "a live check names its project");

    verify_site(&client, &account)?;
    verify_reads(&client)?;

    let key = make_issue(&client, &account)?;
    let steps = verify_issue(&client, &key).and_then(|()| verify_search_walk(&client, &account));
    let deleted = client.send(&DeleteIssue { issue_id_or_key: &key });

    steps?;

    let _reply = deleted?;

    report(&format!("issue {key} deleted"))
}

fn text_of<'value>(settings: &'value Value, field: &str) -> &'value str {
    assert!(!field.is_empty(), "a setting is named");

    let text = settings.get(field).and_then(Value::as_str).unwrap_or_default();

    assert!(!text.contains('\n'), "a setting is one line");

    text
}

fn verify_attachment(client: &Client<Agent>, key: &str) -> Result<(), Error> {
    assert!(!key.is_empty(), "an attachment lands on an issue");

    let parts = [Part {
        bytes: ATTACHMENT_BYTES,
        content_type: "text/plain",
        file_name: Some("live.txt"),
        name: "file",
    }];

    let reply = client.send(&AddAttachment { body: &parts, issue_id_or_key: key })?;
    let mut uploaded = reply.read().ok_or_else(|| reply_missing("the attachment list"))?;
    let attachment = uploaded.next().ok_or_else(|| reply_missing("an attachment"))?;
    let id = attachment.member(Attachment::ID).ok_or_else(|| reply_missing("an attachment id"))?;
    let mut sink = Cursor::new(Vec::new());
    let written = client.download(&GetAttachmentContent { id }, &mut sink)?;

    assert_eq!(sink.get_ref().as_slice(), ATTACHMENT_BYTES, "a download returns what was uploaded");
    assert_eq!(written, 32, "a download counts what it wrote");

    report("attachment uploaded and downloaded through its redirect")
}

fn verify_comment(client: &Client<Agent>, key: &str) -> Result<(), Error> {
    assert!(!key.is_empty(), "a comment lands on an issue");

    let body = json!({
        "body": {
            "content": [{
                "content": [{"text": "A comment from the live check", "type": "text"}],
                "type": "paragraph",
            }],
            "type": "doc",
            "version": 1_i64,
        },
    });

    let reply = client.send(&AddComment { body: &body, issue_id_or_key: key })?;
    let text = reply.value().get("body").map(adf::text_read).unwrap_or_default();

    assert_eq!(text, "A comment from the live check", "the comment reads back as prose");

    report("comment added and flattened")
}

fn verify_comment_walk(client: &Client<Agent>, key: &str) -> Result<(), Error> {
    assert!(!key.is_empty(), "a thread belongs to an issue");

    let paragraph = json!({"content": [{"text": "another", "type": "text"}], "type": "paragraph"});
    let body = json!({"body": {"content": [paragraph], "type": "doc", "version": 1_i64}});

    for _comment_index in 1..COMMENT_COUNT {
        let _reply = client.send(&AddComment { body: &body, issue_id_or_key: key })?;
    }

    let options = get_comments::Options {
        expand: None,
        order_by: Some("created"),
        results_max: Some(1),
        start_at: None,
    };

    let operation = GetComments { issue_id_or_key: key };
    let mut page_count = 0_u64;
    let mut ids: Vec<String> = Vec::new();

    for walked in client.walk_with(&operation, options) {
        for value in walked?.into_values() {
            ids.push(value.get("id").and_then(Value::as_str).unwrap_or_default().to_owned());
        }

        page_count = page_count.strict_add(1);
    }

    ids.sort_unstable();
    ids.dedup();

    assert_eq!(page_count, COMMENT_COUNT, "a page of one comment takes one page a comment");
    assert_eq!(u64::try_from(ids.len()).ok(), Some(COMMENT_COUNT), "each comment arrives once");

    report(&format!("an offset walk crossed {page_count} pages of one comment"))
}

fn verify_issue(client: &Client<Agent>, key: &str) -> Result<(), Error> {
    assert!(!key.is_empty(), "the steps run on one issue");

    let options = get_issue::Options {
        expand: None,
        fail_fast: None,
        fields: Some(&["summary"]),
        fields_by_keys: None,
        properties: None,
        update_history: None,
    };

    let reply = client.send_with(&GetIssue { issue_id_or_key: key }, &options)?;
    let summary = reply.value().pointer("/fields/summary").and_then(Value::as_str);

    assert_eq!(summary, Some(SUMMARY), "the issue reads back as it was written");

    report("issue read back")?;
    verify_comment(client, key)?;
    verify_comment_walk(client, key)?;
    verify_attachment(client, key)
}

fn verify_reads(client: &Client<Agent>) -> Result<(), Error> {
    let user = client.send(&GetCurrentUser)?;
    let fields = client.send(&GetFields)?;
    let field_count = fields.read().map_or(0, Iterator::count);
    let boards = client.send(&GetAllBoards)?;
    let mut project_count = 0_u64;

    assert!(field_count >= 1, "a site has fields");
    assert!(boards.value().is_object(), "a board page is an object");

    for walked in client.walk(&SearchProjects) {
        project_count = project_count.strict_add(walked?.count());
    }

    let account = user.read().and_then(|found| found.member(User::ACCOUNT_ID));
    let options = get_valid_project_key::Options { key: Some("LIVE") };
    let free_key = client.send_with(&GetValidProjectKey, &options)?;

    assert!(account.is_some(), "the current user has an account id");
    assert!(free_key.read().is_some_and(|key| !key.is_empty()), "an unquoted text reply reads");

    report(&format!("reads: {field_count} fields, {project_count} projects"))
}

fn verify_search_walk(client: &Client<Agent>, account: &Account) -> Result<(), Error> {
    assert!(!account.project_key.is_empty(), "a search names its project");

    let jql = format!("project = \"{}\" ORDER BY created ASC", account.project_key);

    let options = search_and_reconsile_issues_using_jql::Options {
        expand: None,
        fail_fast: None,
        fields: Some(&["id"]),
        fields_by_keys: None,
        include_archived_projects: None,
        jql: Some(&jql),
        page_token_next: None,
        properties: None,
        reconcile_issues: None,
        results_max: Some(SEARCH_PAGE_ITEM_COUNT),
    };

    let mut ids: Vec<String> = Vec::new();
    let mut page_count = 0_u64;

    for walked in client.walk_with(&SearchAndReconsileIssuesUsingJQL, options) {
        for value in walked?.into_values() {
            ids.push(value.get("id").and_then(Value::as_str).unwrap_or_default().to_owned());
        }

        page_count = page_count.strict_add(1);
    }

    let item_count = ids.len();

    ids.sort_unstable();
    ids.dedup();

    assert_eq!(ids.len(), item_count, "a token walk hands back each issue once");
    assert!(ids.iter().all(|id| !id.is_empty()), "each issue names its id");

    report(&format!("a token walk crossed {page_count} pages and found {item_count} issues"))
}

fn verify_site(client: &Client<Agent>, account: &Account) -> Result<(), Error> {
    assert!(!account.site.is_empty(), "a live check names the site it expects");

    let reply = client.send(&GetServerInfo)?;
    let information = reply.read().ok_or_else(|| reply_missing("the server information"))?;
    let base = information.member(ServerInformation::BASE_URL).unwrap_or_default();

    if base.trim_end_matches('/') != account.site {
        return Err(Error::Input(format!(
            "these credentials reach {base}, not {}, so nothing else runs",
            account.site,
        )));
    }

    report(&format!("site confirmed: {base}"))
}
