#[derive(Clone, Copy, Debug)]
pub struct SearchProjects;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub action: Option<&'options str>,
    pub category_id: Option<i64>,
    pub expand: Option<&'options str>,
    pub id: Option<&'options [i64]>,
    pub keys: Option<&'options [&'options str]>,
    pub order_by: Option<&'options str>,
    pub properties: Option<&'options [&'options str]>,
    pub property_query: Option<&'options str>,
    pub query: Option<&'options str>,
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
    pub status: Option<&'options [&'options str]>,
    pub type_key: Option<&'options str>,
}

impl crate::Operation for SearchProjects {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/project/search";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PageBeanProject>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("action", options.action);
        request.query_number("categoryId", options.category_id);
        request.query_text("expand", options.expand);
        request.query_numbers("id", options.id);
        request.query_texts("keys", options.keys);
        request.query_text("orderBy", options.order_by);
        request.query_texts("properties", options.properties);
        request.query_text("propertyQuery", options.property_query);
        request.query_text("query", options.query);
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);
        request.query_texts("status", options.status);
        request.query_text("typeKey", options.type_key);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for SearchProjects {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::Project>;
}
