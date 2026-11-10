#[derive(Clone, Copy, Debug)]
pub struct GetScreens;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub id: Option<&'options [i64]>,
    pub order_by: Option<&'options str>,
    pub query_string: Option<&'options str>,
    pub results_max: Option<i64>,
    pub scope: Option<&'options [&'options str]>,
    pub start_at: Option<i64>,
}

impl crate::Operation for GetScreens {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/screens";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PageBeanScreen>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_numbers("id", options.id);
        request.query_text("orderBy", options.order_by);
        request.query_text("queryString", options.query_string);
        request.query_number("maxResults", options.results_max);
        request.query_texts("scope", options.scope);
        request.query_number("startAt", options.start_at);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetScreens {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::Screen>;
}
