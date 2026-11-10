#[derive(Clone, Copy, Debug)]
pub struct GetCustomFieldContextsForProjectsAndIssueTypes<'call> {
    pub body: &'call serde_json::Value,
    pub field_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
}

impl crate::Operation for GetCustomFieldContextsForProjectsAndIssueTypes<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/api/3/field/{fieldId}/context/mapping";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::PageBeanContextForProjectAndIssueType>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("fieldId", self.field_id)?;
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
