#[derive(Clone, Copy, Debug)]
pub struct GetQueue<'call> {
    pub queue_id: i64,
    pub service_desk_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub include_count: Option<bool>,
}

impl crate::Operation for GetQueue<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/servicedeskapi/servicedesk/{serviceDeskId}/queue/{queueId}";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::QueueDTO>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_boolean("includeCount", options.include_count);
        request.path_number("queueId", self.queue_id)?;
        request.path_text("serviceDeskId", self.service_desk_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
