#[derive(Clone, Copy, Debug)]
pub struct GetCustomerRequests;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub approval_status: Option<&'options str>,
    pub expand: Option<&'options [&'options str]>,
    pub limit: Option<i64>,
    pub organization_id: Option<i64>,
    pub request_ownership: Option<&'options [&'options str]>,
    pub request_status: Option<&'options str>,
    pub request_type_id: Option<i64>,
    pub search_term: Option<&'options str>,
    pub service_desk_id: Option<i64>,
    pub start: Option<i64>,
}

impl crate::Operation for GetCustomerRequests {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/servicedeskapi/request";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PagedDTOCustomerRequestDTO>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("approvalStatus", options.approval_status);
        request.query_texts("expand", options.expand);
        request.query_number("limit", options.limit);
        request.query_number("organizationId", options.organization_id);
        request.query_texts("requestOwnership", options.request_ownership);
        request.query_text("requestStatus", options.request_status);
        request.query_number("requestTypeId", options.request_type_id);
        request.query_text("searchTerm", options.search_term);
        request.query_number("serviceDeskId", options.service_desk_id);
        request.query_number("start", options.start);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetCustomerRequests {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "start", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::CustomerRequestDTO>;
}
