#[derive(Clone, Copy, Debug)]
pub struct GetIssue<'call> {
    pub issue_id_or_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options str>,
    pub fields: Option<&'options [&'options str]>,
    pub update_history: Option<bool>,
}

impl crate::Operation for GetIssue<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/agile/1.0/issue/{issueIdOrKey}";

    type Options<'options> = Options<'options>;
    type Reply = crate::Any;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("expand", options.expand);
        request.query_texts("fields", options.fields);
        request.path_text("issueIdOrKey", self.issue_id_or_key)?;
        request.query_boolean("updateHistory", options.update_history);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
