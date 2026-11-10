#[derive(Clone, Copy, Debug)]
pub struct ConnectToForgeMigrationTaskSubmissionResourceSubmitTaskPost<'call> {
    pub connect_key: &'call str,
    pub jira_issue_fields_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub retrigger_completed_migration: Option<bool>,
}

impl crate::Operation for ConnectToForgeMigrationTaskSubmissionResourceSubmitTaskPost<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;

    const PATH: &'static str =
        "/rest/atlassian-connect/1/migration/{connectKey}/{jiraIssueFieldsKey}/task";

    type Options<'options> = Options;
    type Reply = crate::Nothing;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("connectKey", self.connect_key)?;
        request.path_text("jiraIssueFieldsKey", self.jira_issue_fields_key)?;
        request.query_boolean("retriggerCompletedMigration", options.retrigger_completed_migration);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
