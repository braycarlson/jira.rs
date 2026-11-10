#[derive(Clone, Copy, Debug)]
pub struct SchemaList<'call> {
    pub workspace_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub include_counts: Option<bool>,
    pub query: Option<&'options str>,
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
}

impl crate::Operation for SchemaList<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/jsm/assets/workspace/{workspaceId}/v1/objectschema/list";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::ObjectSchemaList>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_boolean("includeCounts", options.include_counts);
        request.query_text("query", options.query);
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);
        request.path_text("workspaceId", self.workspace_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for SchemaList<'_> {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::ObjectSchema>;
}
