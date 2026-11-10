#[derive(Clone, Copy, Debug)]
pub struct GetPrioritySchemes;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options str>,
    pub only_default: Option<bool>,
    pub order_by: Option<&'options str>,
    pub priority_id: Option<&'options [i64]>,
    pub results_max: Option<&'options str>,
    pub scheme_id: Option<&'options [i64]>,
    pub scheme_name: Option<&'options str>,
    pub start_at: Option<&'options str>,
}

impl crate::Operation for GetPrioritySchemes {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/priorityscheme";

    type Options<'options> = Options<'options>;

    type Reply =
        crate::Object<super::shape::PageBeanPrioritySchemeWithPaginatedPrioritiesAndProjects>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("expand", options.expand);
        request.query_boolean("onlyDefault", options.only_default);
        request.query_text("orderBy", options.order_by);
        request.query_numbers("priorityId", options.priority_id);
        request.query_text("maxResults", options.results_max);
        request.query_numbers("schemeId", options.scheme_id);
        request.query_text("schemeName", options.scheme_name);
        request.query_text("startAt", options.start_at);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetPrioritySchemes {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::PrioritySchemeWithPaginatedPrioritiesAndProjects>;
}
