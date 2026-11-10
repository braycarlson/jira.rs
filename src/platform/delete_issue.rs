#[derive(Clone, Copy, Debug)]
pub struct DeleteIssue<'call> {
    pub issue_id_or_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub delete_subtasks: Option<&'options str>,
}

impl crate::Operation for DeleteIssue<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/api/3/issue/{issueIdOrKey}";

    type Options<'options> = Options<'options>;
    type Reply = crate::Nothing;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("deleteSubtasks", options.delete_subtasks);
        request.path_text("issueIdOrKey", self.issue_id_or_key)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
