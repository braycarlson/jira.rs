#[derive(Clone, Copy, Debug)]
pub struct GetAllDashboards;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub filter: Option<&'options str>,
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
}

impl crate::Operation for GetAllDashboards {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/dashboard";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PageOfDashboards>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("filter", options.filter);
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetAllDashboards {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/dashboards"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::Dashboard>;
}
