#[derive(Clone, Copy, Debug)]
pub struct GetEditIssueMeta<'call> {
    pub issue_id_or_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub override_editable_flag: Option<bool>,
    pub override_screen_security: Option<bool>,
}

impl crate::Operation for GetEditIssueMeta<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/issue/{issueIdOrKey}/editmeta";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::IssueUpdateMetadata>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("issueIdOrKey", self.issue_id_or_key)?;
        request.query_boolean("overrideEditableFlag", options.override_editable_flag);
        request.query_boolean("overrideScreenSecurity", options.override_screen_security);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
