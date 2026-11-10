#[derive(Clone, Copy, Debug)]
pub struct GetAllRequestTypes;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options [&'options str]>,
    pub include_hidden_request_types_in_search: Option<bool>,
    pub limit: Option<i64>,
    pub restriction_status: Option<&'options str>,
    pub search_query: Option<&'options str>,
    pub service_desk_id: Option<&'options [i64]>,
    pub start: Option<i64>,
}

impl crate::Operation for GetAllRequestTypes {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/servicedeskapi/requesttype";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PagedDTORequestTypeDTO>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_texts("expand", options.expand);

        request.query_boolean(
            "includeHiddenRequestTypesInSearch",
            options.include_hidden_request_types_in_search,
        );

        request.query_number("limit", options.limit);
        request.query_text("restrictionStatus", options.restriction_status);
        request.query_text("searchQuery", options.search_query);
        request.query_numbers("serviceDeskId", options.service_desk_id);
        request.query_number("start", options.start);
        request.header("X-ExperimentalApi", "opt-in");

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetAllRequestTypes {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "start", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::RequestTypeDTO>;
}
