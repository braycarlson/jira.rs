#[derive(Clone, Copy, Debug)]
pub struct GetCustomFieldConfiguration<'call> {
    pub field_id_or_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub field_context_id: Option<&'options [i64]>,
    pub id: Option<&'options [i64]>,
    pub issue_id: Option<i64>,
    pub issue_type_id: Option<&'options str>,
    pub project_key_or_id: Option<&'options str>,
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
}

impl crate::Operation for GetCustomFieldConfiguration<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/app/field/{fieldIdOrKey}/context/configuration";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PageBeanContextualConfiguration>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_numbers("fieldContextId", options.field_context_id);
        request.path_text("fieldIdOrKey", self.field_id_or_key)?;
        request.query_numbers("id", options.id);
        request.query_number("issueId", options.issue_id);
        request.query_text("issueTypeId", options.issue_type_id);
        request.query_text("projectKeyOrId", options.project_key_or_id);
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetCustomFieldConfiguration<'_> {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::ContextualConfiguration>;
}
