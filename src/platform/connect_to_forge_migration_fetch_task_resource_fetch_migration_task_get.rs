#[derive(Clone, Copy, Debug)]
pub struct ConnectToForgeMigrationFetchTaskResourceFetchMigrationTaskGet<'call> {
    pub connect_key: &'call str,
    pub jira_issue_fields_key: &'call str,
}

impl crate::Operation for ConnectToForgeMigrationFetchTaskResourceFetchMigrationTaskGet<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;

    const PATH: &'static str =
        "/rest/atlassian-connect/1/migration/{connectKey}/{jiraIssueFieldsKey}/task";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::TaskProgress>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("connectKey", self.connect_key)?;
        request.path_text("jiraIssueFieldsKey", self.jira_issue_fields_key)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
