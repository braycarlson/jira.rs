use std::io::{self, Seek, Write};
use std::process::ExitCode;

use jira::platform::shape::{IssueBean, Project};
use jira::platform::{AddComment, GetIssue, SearchProjects};
use jira::{Client, Credentials, Error, Method, Request, Response, Transport, Wait, adf};
use serde_json::{Value, json};

#[derive(Debug)]
struct Offline {
    issue: Value,
    projects: Value,
}

impl Offline {
    fn new() -> Self {
        let issue = json!({
            "id": "101",
            "key": "DEMO-1",
            "fields": {
                "description": {
                    "type": "doc",
                    "content": [{
                        "type": "paragraph",
                        "content": [{"type": "text", "text": "The login times out."}],
                    }],
                },
                "summary": "Checkout stalls",
            },
        });

        let projects = json!({
            "isLast": true,
            "total": 2_i64,
            "values": [{"key": "DEMO"}, {"key": "SHOP"}],
        });

        assert!(issue.get("key").is_some(), "the issue carries a key");
        assert!(projects.get("values").is_some(), "the page carries its projects");

        Self { issue, projects }
    }
}

impl Transport for Offline {
    fn fetch(&self, request: &Request, bytes_max: u64) -> Result<Response, Error> {
        assert!(bytes_max >= 1, "a body may hold at least one byte");
        assert!(request.url().starts_with("https://"), "every request is encrypted");

        let (status, body) = if request.method() == Method::Post {
            (201, json!({"id": "9"}))
        } else if request.url().contains("/project/search") {
            (200, self.projects.clone())
        } else {
            (200, self.issue.clone())
        };

        Ok(Response {
            body: serde_json::to_vec(&body).unwrap_or_default(),
            is_near_limit: false,
            location: None,
            rate_limit_reason: None,
            retry_after_seconds: None,
            status,
            streamed_count: 0,
        })
    }

    fn stream<Sink>(&self, request: &Request, sink: &mut Sink) -> Result<Response, Error>
    where
        Sink: Seek + Write,
    {
        assert!(request.url().starts_with("https://"), "every request is encrypted");

        let response = self.fetch(request, 1_024)?;

        sink.write_all(&response.body)
            .map_err(|cause| Error::Input(format!("cannot write the download: {cause}")))?;

        assert!(response.status >= 200, "the example only ever succeeds");

        Ok(response)
    }

    fn wait(&self, wait: &Wait<'_>) -> Result<(), Error> {
        assert!(!wait.reason.is_empty(), "a wait says why");
        assert!(self.projects.is_object(), "the example holds its fixtures");

        Ok(())
    }
}

fn main() -> ExitCode {
    let Err(failure) = run() else {
        return ExitCode::SUCCESS;
    };

    assert!(!failure.message().is_empty(), "a failure says why");
    assert!(failure.status().is_none(), "the example only ever succeeds");

    if writeln!(io::stderr().lock(), "{failure}").is_err() {
        return ExitCode::from(2);
    }

    ExitCode::FAILURE
}

fn report(line: &str) -> Result<(), Error> {
    assert!(!line.is_empty(), "a report says something");
    assert!(!line.contains('\n'), "a report is one line");

    writeln!(io::stdout().lock(), "{line}")
        .map_err(|cause| Error::Input(format!("cannot write to standard output: {cause}")))
}

fn run() -> Result<(), Error> {
    let credentials = Credentials::Token {
        cloud_id: "11111111-2222-3333-4444-555555555555",
        email: "user@example.com",
        token: "token",
    };

    let client = Client::new(Offline::new(), &credentials)?;
    let reply = client.send(&GetIssue { issue_id_or_key: "DEMO-1" })?;
    let issue = reply.read().ok_or_else(|| Error::Reply("the issue is no object".to_owned()))?;
    let fields = issue.member(IssueBean::FIELDS);
    let description = fields.and_then(|found| found.member("description"));

    report(issue.member(IssueBean::KEY).unwrap_or("no key"))?;
    report(&description.map(adf::text_read).unwrap_or_default())?;

    for walked in client.walk(&SearchProjects) {
        let found = walked?;

        assert!(found.count() >= 1, "the example holds projects");

        for project in found.items() {
            report(project.member(Project::KEY).unwrap_or("no key"))?;
        }
    }

    let body = json!({"body": {"type": "doc", "version": 1_i64, "content": []}});
    let created = client.send(&AddComment { body: &body, issue_id_or_key: "DEMO-1" })?;
    let identifier = created.value().get("id").and_then(Value::as_str);

    assert!(identifier.is_some(), "the example reply names the comment");

    report(identifier.unwrap_or("no id"))
}
