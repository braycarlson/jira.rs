#[derive(Clone, Copy, Debug)]
pub struct GetRequestTypeGroups<'call> {
    pub service_desk_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub limit: Option<i64>,
    pub start: Option<i64>,
}

impl crate::Operation for GetRequestTypeGroups<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/servicedeskapi/servicedesk/{serviceDeskId}/requesttypegroup";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::PagedDTORequestTypeGroupDTO>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_number("limit", options.limit);
        request.path_text("serviceDeskId", self.service_desk_id)?;
        request.query_number("start", options.start);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetRequestTypeGroups<'_> {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "start", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::RequestTypeGroupDTO>;
}
