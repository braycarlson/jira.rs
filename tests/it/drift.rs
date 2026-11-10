use jira::Error;
use jira::assets::shape::{ObjectType, Progress};
use jira::assets::{ObjectTypeFind, ProgressImport};
use jira::platform::shape::{
    BulkChangelogResponseBean,
    BulkOperationProgress,
    Changelog,
    DashboardGadget,
    DashboardGadgetPosition,
    ExportArchivedIssuesTaskProgressResponse,
    FieldCreateMetadata,
    IssueChangeLog,
    PolicySummary,
    PrioritySchemeID,
    TaskProgressBeanJSONNode,
    Webhook,
};
use jira::platform::{
    AddGadget,
    CreatePriorityScheme,
    DeleteRemoteIssueLinkByGlobalID,
    ExportArchivedIssues,
    GetBulkChangelogs,
    GetBulkOperationProgress,
    GetCreateIssueMetaIssueTypeID,
    GetDraftWorkflow,
    GetDynamicWebhooksForApp,
    GetIDsOfWorklogsModifiedSince,
    GetWorkflow,
    ListPolicies,
    RemoveTemplate,
    SetIssueNavigatorDefaultColumns,
    SetUserColumns,
    get_draft_workflow,
    get_workflow,
};
use jira::service_desk::shape::{ArticleDTO, PagedDTOArticleDTO, SourceDTO};
use jira::service_desk::{GetArticles, SetRequestTypeProperty};
use jira::software::{DeleteBuildsByProperty, DeleteRemoteLinksByProperty};
use serde_json::{Value, json};

use crate::fake::{answer, client};

const EMPTY: &Value = &Value::Null;

#[test]
fn a_bulk_task_reads_the_epoch_times_jira_sends() {
    let body = json!({
        "created": 1_794_020_249_835_i64,
        "started": 1_794_020_249_911_i64,
        "status": "COMPLETE",
        "taskId": "10176",
        "updated": 1_794_020_250_199_i64,
    });

    let jira = client(vec![answer(200, &body)]);
    let reply = jira.send(&GetBulkOperationProgress { task_id: "10176" }).expect("a reply");
    let progress = reply.read().expect("an object");

    assert_eq!(progress.member(BulkOperationProgress::CREATED), Some(1_794_020_249_835), "created");
    assert_eq!(progress.member(BulkOperationProgress::STARTED), Some(1_794_020_249_911), "started");
    assert_eq!(progress.member(BulkOperationProgress::UPDATED), Some(1_794_020_250_199), "updated");
}

#[test]
fn a_changelog_time_reads_in_both_forms_jira_sends() {
    let epoch = 1_794_020_133_679_i64;
    let text = "2026-11-06T20:55:33.679-0700";
    let bulk = json!({"issueChangeLogs": [{"changeHistories": [{"created": epoch}]}]});
    let single = json!({"issueChangeLogs": [{"changeHistories": [{"created": text}]}]});

    assert_eq!(changelog_created(&bulk), json!(epoch), "the bulk endpoint sends an integer");
    assert_eq!(changelog_created(&single), json!(text), "the single endpoint sends text");
}

#[test]
fn a_child_object_type_reads_the_parent_id_jira_sends_as_text() {
    let body = json!({"id": "56", "name": "Sweep Child", "parentObjectTypeId": "55"});
    let jira = client(vec![answer(200, &body)]);
    let reply = jira.send(&ObjectTypeFind { id: "56", workspace_id: "w" }).expect("a reply");
    let kind = reply.read().expect("an object");

    assert_eq!(kind.member(ObjectType::PARENT_OBJECT_TYPE_ID), Some("55"), "the parent id");
}

#[test]
fn a_column_write_sends_the_json_jira_takes_rather_than_the_form_it_documents() {
    let body = json!({"columns": ["summary", "status"]});
    let jira = client(vec![answer(204, &Value::Null), answer(204, &Value::Null)]);
    let user = jira.send(&SetUserColumns { body: &body }).expect("a write");
    let navigator = jira.send(&SetIssueNavigatorDefaultColumns { body: &body }).expect("a write");
    let seen = jira.network().seen();
    let sent = serde_json::to_vec(&body).expect("a body");

    assert!(user.read().is_some(), "an empty answer reads as nothing");
    assert!(navigator.read().is_some(), "an empty answer reads as nothing");

    for write in &seen {
        assert_eq!(write.content_type.as_deref(), Some("application/json"), "not a form");
        assert_eq!(write.body, sent, "the columns are the body");
    }
}

#[test]
fn a_create_meta_walk_reads_the_array_jira_sends() {
    let first = create_meta_page("summary", 0);
    let second = create_meta_page("labels", 1);
    let jira = client(vec![answer(200, &first), answer(200, &second)]);
    let called = GetCreateIssueMetaIssueTypeID { issue_type_id: "10001", project_id_or_key: "KAN" };
    let mut names = Vec::new();

    for page in jira.walk(&called) {
        let held = page.expect("a page");

        let found = held.items().filter_map(|field| field.member(FieldCreateMetadata::FIELD_ID));

        names.extend(found.map(str::to_owned));
    }

    assert_eq!(names, ["summary", "labels"], "every page was read through the real array");
    assert_eq!(jira.network().seen().len(), 2, "the second page was asked for");
}

#[test]
fn a_delete_by_property_sends_the_terms_its_caller_names() {
    let jira = client(vec![answer(202, &Value::Null), answer(202, &Value::Null)]);
    let builds = DeleteBuildsByProperty { properties: &[("accountId", "a b"), ("env", "prod")] };
    let links = DeleteRemoteLinksByProperty { properties: &[("accountId", "x")] };

    let reply = jira.send(&builds).expect("a delete");
    let linked = jira.send(&links).expect("a delete");
    let seen = jira.network().seen();

    assert_eq!(reply.status(), 202, "the delete was accepted");
    assert_eq!(linked.status(), 202, "and so was the second");
    assert!(seen[0].url.ends_with("/bulkByProperties?accountId=a%20b&env=prod"), "{}", seen[0].url);

    assert!(
        seen[1].url.ends_with("/bulkByProperties?accountId=x"),
        "no params term: {}",
        seen[1].url,
    );

    let refusing = client(vec![answer(202, &Value::Null)]);
    let failure = refusing.send(&DeleteBuildsByProperty { properties: &[] });

    assert!(matches!(failure, Err(Error::Input(_))), "Jira requires at least one property");
    assert!(refusing.network().seen().is_empty(), "so nothing is sent");
}

#[test]
fn a_delete_jira_refuses_without_a_type_declares_one_with_no_body() {
    let jira = client(vec![answer(204, &Value::Null)]);
    let called = DeleteRemoteIssueLinkByGlobalID { global_id: "a=b", issue_id_or_key: "KAN-1" };
    let reply = jira.send(&called).expect("a delete");
    let seen = jira.network().seen();

    assert!(reply.read().is_some(), "an empty answer reads as nothing");
    assert_eq!(seen[0].content_type.as_deref(), Some("application/json"), "the type");
    assert!(seen[0].body.is_empty(), "nothing follows the headers");
}

#[test]
fn a_delete_without_a_body_still_declares_its_type_where_jira_demands_one() {
    let jira = client(vec![answer(404, &Value::Null)]);
    let refused = jira.send(&RemoveTemplate { template_key: "absent" }).expect_err("no template");
    let seen = jira.network().seen();

    assert_eq!(refused.status(), Some(404), "the refusal is Jira's rather than a 415");
    assert_eq!(seen[0].content_type.as_deref(), Some("application/json"), "the type");
    assert!(seen[0].body.is_empty(), "nothing follows the headers");
}

#[test]
fn a_gadget_position_reads_the_names_jira_sends() {
    let body =
        json!({"color": "blue", "id": 10022_i32, "position": {"column": 2_i32, "row": 1_i32}});

    let jira = client(vec![answer(200, &body)]);
    let reply = jira.send(&AddGadget { body: EMPTY, dashboard_id: 10001 }).expect("a reply");
    let gadget = reply.read().expect("an object");
    let position = gadget.member(DashboardGadget::POSITION).expect("a position");

    assert_eq!(gadget.member(DashboardGadget::ID), Some(10022), "the gadget");
    assert_eq!(position.member(DashboardGadgetPosition::COLUMN), Some(2), "the column");
    assert_eq!(position.member(DashboardGadgetPosition::ROW), Some(1), "the row");
}

#[test]
fn a_knowledge_walk_carries_every_term_of_the_link_jira_returns() {
    let link = "https://api.atlassian.com/ex/jira/x/rest/servicedeskapi/knowledgebase/article";
    let next = format!("{link}?query=sweep&limit=1&start=1&cursor=_t_Wy%3D%3D_h_W10%3D&prev=false");

    let first = json!({
        "_links": {"next": next},
        "isLastPage": false,
        "start": 0_i64,
        "values": [{"title": "three"}],
    });

    let second =
        json!({"_links": {}, "isLastPage": true, "start": 1_i64, "values": [{"title": "two"}]});

    let jira = client(vec![answer(200, &first), answer(200, &second)]);
    let mut titles = Vec::new();

    for page in jira.walk(&GetArticles { highlight: false, query: "sweep" }) {
        let held = page.expect("a page");

        titles.extend(
            held.items().filter_map(|article| article.member(ArticleDTO::TITLE)).map(str::to_owned),
        );
    }

    let seen = jira.network().seen();

    assert_eq!(titles, ["three", "two"], "each page moved on");
    assert!(seen[1].url.contains("cursor=_t_Wy%3D%3D_h_W10%3D"), "the cursor: {}", seen[1].url);
    assert!(seen[1].url.contains("start=1"), "the term Jira answers 500 without: {}", seen[1].url);
}

#[test]
fn a_policy_walk_follows_the_cursor_rather_than_the_legacy_offset() {
    let first = json!({
        "isLast": false,
        "nextPageCursor": "alphabetical:one",
        "values": [{"id": "1"}],
    });

    let second = json!({"isLast": true, "values": [{"id": "2"}]});
    let jira = client(vec![answer(200, &first), answer(200, &second)]);
    let mut ids = Vec::new();

    for page in jira.walk(&ListPolicies) {
        let held = page.expect("a page");

        ids.extend(
            held.items().filter_map(|policy| policy.member(PolicySummary::ID)).map(str::to_owned),
        );
    }

    let seen = jira.network().seen();

    assert_eq!(ids, ["1", "2"], "each page moved on");
    assert!(seen[1].url.contains("cursor=alphabetical%3Aone"), "the cursor: {}", seen[1].url);
    assert!(!seen[1].url.contains("startAt"), "the offset Jira refuses: {}", seen[1].url);
}

#[test]
fn a_request_type_property_sends_the_value_the_specification_forgot() {
    let jira = client(vec![answer(200, &json!({"key": "sweep.property"}))]);
    let value = json!({"sweep": 1_i64});

    let called = SetRequestTypeProperty {
        body: &value,
        property_key: "sweep.property",
        request_type_id: 7,
        service_desk_id: "2",
    };

    let reply = jira.send(&called).expect("a write");
    let seen = jira.network().seen();

    assert!(reply.value().is_object(), "the answer is read whole");
    assert_eq!(seen[0].body, br#"{"sweep":1}"#, "the value travels as the body");
    assert_eq!(seen[0].content_type.as_deref(), Some("application/json"), "the type");
}

#[test]
fn a_scheme_workflow_read_keeps_a_list_and_an_object_alike() {
    let list = json!([{"defaultMapping": true, "issueTypes": [], "workflow": "jira"}]);
    let single = json!({"defaultMapping": false, "issueTypes": ["10007"], "workflow": "sweep"});
    let jira = client(vec![answer(200, &list), answer(200, &single), answer(200, &list)]);
    let every = GetWorkflow { id: 10002 };
    let draft = GetDraftWorkflow { id: 10002 };

    let named =
        get_workflow::Options { return_draft_if_exists: None, workflow_name: Some("sweep") };

    let open = get_draft_workflow::Options::default();
    let all = jira.send(&every).expect("every mapping");
    let one = jira.send_with(&every, &named).expect("one mapping");
    let drafted = jira.send_with(&draft, &open).expect("every drafted mapping");

    assert_eq!(all.read(), Some(&list), "with no name Jira answers a list");
    assert_eq!(one.read(), Some(&single), "with a name Jira answers one mapping");
    assert_eq!(drafted.read(), Some(&list), "a draft answers the same way");
}

#[test]
fn a_task_result_reads_whatever_json_it_holds() {
    let body = json!({"id": "10001", "task": {"result": "the task result"}});
    let jira = client(vec![answer(200, &body)]);
    let reply = jira.send(&CreatePriorityScheme { body: EMPTY }).expect("a reply");
    let created = reply.read().expect("an object");
    let task = created.member(PrioritySchemeID::TASK).expect("a task");
    let result = task.member(TaskProgressBeanJSONNode::RESULT);

    assert_eq!(result, Some(&json!("the task result")), "text reads as what it is");
    assert_eq!(created.member(PrioritySchemeID::ID), Some("10001"), "the id beside it");
}

#[test]
fn a_webhook_expiry_reads_in_either_form_the_specification_disagrees_on() {
    let epoch = json!(1_559_392_950_000_i64);
    let text = json!("2019-06-01T12:42:30.000+0000");

    assert_eq!(webhook_expiry(&epoch), epoch, "the schema says integer");
    assert_eq!(webhook_expiry(&text), text, "the examples say text");
}

#[test]
fn a_worklog_walk_follows_the_page_address_jira_returns() {
    let next = "https://api.atlassian.com/rest/api/3/worklog/updated?since=1438013693136";
    let first = json!({"lastPage": false, "nextPage": next, "values": [{"worklogId": 1_i64}]});
    let second = json!({"lastPage": true, "values": [{"worklogId": 2_i64}]});
    let jira = client(vec![answer(200, &first), answer(200, &second)]);
    let pages: Vec<_> = jira.walk(&GetIDsOfWorklogsModifiedSince).collect();
    let seen = jira.network().seen();

    assert_eq!(pages.len(), 2, "both pages");
    assert!(pages.iter().all(Result::is_ok), "each read");
    assert!(seen[1].url.ends_with("/worklog/updated?since=1438013693136"), "{}", seen[1].url);
}

#[test]
fn an_article_names_the_page_the_specification_never_mentions() {
    let body = json!({
        "isLastPage": true,
        "values": [{"source": {"pageId": "196732", "spaceKey": "SD", "type": "confluence"}}],
    });

    let jira = client(vec![answer(200, &body)]);
    let called = GetArticles { highlight: false, query: "software" };
    let reply = jira.send(&called).expect("a reply");
    let page = reply.read().expect("an object");
    let rows = page.member(PagedDTOArticleDTO::VALUES).expect("a list");
    let article = rows.into_iter().next().expect("a row");
    let source = article.member(ArticleDTO::SOURCE).expect("a source");

    assert_eq!(source.member(SourceDTO::PAGE_ID), Some("196732"), "the id ViewArticle takes");
    assert_eq!(source.member(SourceDTO::SPACE_KEY), Some("SD"), "the space");
}

#[test]
fn an_export_task_reads_the_epoch_time_jira_sends() {
    let body = json!({"status": "ENQUEUED", "submittedTime": 1_794_019_504_336_i64, "taskId": "1"});
    let jira = client(vec![answer(202, &body)]);
    let reply = jira.send(&ExportArchivedIssues { body: EMPTY }).expect("a reply");
    let task = reply.read().expect("an object");
    let submitted = task.member(ExportArchivedIssuesTaskProgressResponse::SUBMITTED_TIME);
    let status = task.member(ExportArchivedIssuesTaskProgressResponse::STATUS);

    assert_eq!(submitted, Some(1_794_019_504_336), "the time Jira sent");
    assert_eq!(status, Some("ENQUEUED"), "the text beside it");
}

#[test]
fn an_import_progress_reads_the_epoch_times_jira_sends() {
    let body = json!({"finishedDate": 0_i64, "startDate": 1_794_033_350_197_i64});
    let jira = client(vec![answer(200, &body)]);
    let reply = jira.send(&ProgressImport { id: "1", workspace_id: "w" }).expect("a reply");
    let progress = reply.read().expect("an object");

    assert_eq!(progress.member(Progress::FINISHED_DATE), Some(0), "not finished");
    assert_eq!(progress.member(Progress::START_DATE), Some(1_794_033_350_197), "started");
}

fn changelog_created(body: &Value) -> Value {
    let jira = client(vec![answer(200, body)]);
    let reply = jira.send(&GetBulkChangelogs { body: EMPTY }).expect("a reply");
    let bean = reply.read().expect("an object");
    let mut logs = bean.member(BulkChangelogResponseBean::ISSUE_CHANGE_LOGS).expect("the logs");
    let log = logs.next().expect("one log");
    let mut histories = log.member(IssueChangeLog::CHANGE_HISTORIES).expect("the histories");
    let history = histories.next().expect("one history");

    assert_eq!(jira.network().seen().len(), 1, "one reply holds the whole changelog");

    history.member(Changelog::CREATED).expect("a time").clone()
}

fn create_meta_page(name: &str, start: i32) -> Value {
    assert!(!name.is_empty(), "a field is named");
    assert!(start >= 0_i32, "a page starts at or past the first item");

    json!({"fields": [{"fieldId": name}], "maxResults": 1_i32, "startAt": start, "total": 2_i32})
}

fn webhook_expiry(expiry: &Value) -> Value {
    let page = json!({
        "isLast": true,
        "maxResults": 1_i32,
        "startAt": 0_i32,
        "total": 1_i32,
        "values": [{"expirationDate": expiry, "id": 1_i32}],
    });

    let jira = client(vec![answer(200, &page)]);
    let mut pages = jira.walk(&GetDynamicWebhooksForApp);
    let held = pages.next().expect("a page").expect("a good page");
    let webhook = held.items().next().expect("a webhook");
    let following = pages.next();

    assert!(following.is_none(), "the last page ends the walk");

    webhook.member(Webhook::EXPIRATION_DATE).expect("an expiry").clone()
}
