#[derive(Clone, Copy, Debug)]
pub struct RemoveIssueTypesFromContext<'call> {
    pub body: &'call serde_json::Value,
    pub context_id: i64,
    pub field_id: &'call str,
}

impl crate::Operation for RemoveIssueTypesFromContext<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/api/3/field/{fieldId}/context/{contextId}/issuetype/remove";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("contextId", self.context_id)?;
        request.path_text("fieldId", self.field_id)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
