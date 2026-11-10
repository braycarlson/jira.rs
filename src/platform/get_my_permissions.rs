#[derive(Clone, Copy, Debug)]
pub struct GetMyPermissions;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub comment_id: Option<&'options str>,
    pub issue_id: Option<&'options str>,
    pub issue_key: Option<&'options str>,
    pub permissions: Option<&'options str>,
    pub project_configuration_uuid: Option<&'options str>,
    pub project_id: Option<&'options str>,
    pub project_key: Option<&'options str>,
    pub project_uuid: Option<&'options str>,
}

impl crate::Operation for GetMyPermissions {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/mypermissions";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::Permissions>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("commentId", options.comment_id);
        request.query_text("issueId", options.issue_id);
        request.query_text("issueKey", options.issue_key);
        request.query_text("permissions", options.permissions);
        request.query_text("projectConfigurationUuid", options.project_configuration_uuid);
        request.query_text("projectId", options.project_id);
        request.query_text("projectKey", options.project_key);
        request.query_text("projectUuid", options.project_uuid);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
