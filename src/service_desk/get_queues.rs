#[derive(Clone, Copy, Debug)]
pub struct GetQueues<'call> {
    pub service_desk_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub include_count: Option<bool>,
    pub limit: Option<i64>,
    pub start: Option<i64>,
}

impl crate::Operation for GetQueues<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/servicedeskapi/servicedesk/{serviceDeskId}/queue";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::PagedDTOQueueDTO>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_boolean("includeCount", options.include_count);
        request.query_number("limit", options.limit);
        request.path_text("serviceDeskId", self.service_desk_id)?;
        request.query_number("start", options.start);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetQueues<'_> {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "start", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::QueueDTO>;
}
