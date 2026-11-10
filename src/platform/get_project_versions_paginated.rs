#[derive(Clone, Copy, Debug)]
pub struct GetProjectVersionsPaginated<'call> {
    pub project_id_or_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options str>,
    pub order_by: Option<&'options str>,
    pub query: Option<&'options str>,
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
    pub status: Option<&'options str>,
}

impl crate::Operation for GetProjectVersionsPaginated<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/project/{projectIdOrKey}/version";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PageBeanVersion>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("expand", options.expand);
        request.query_text("orderBy", options.order_by);
        request.path_text("projectIdOrKey", self.project_id_or_key)?;
        request.query_text("query", options.query);
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);
        request.query_text("status", options.status);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetProjectVersionsPaginated<'_> {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::Version>;
}
