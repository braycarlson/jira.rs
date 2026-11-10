#[derive(Clone, Copy, Debug)]
pub struct GetRequestTypeFields<'call> {
    pub request_type_id: i64,
    pub service_desk_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options [&'options str]>,
}

impl crate::Operation for GetRequestTypeFields<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;

    const PATH: &'static str =
        "/rest/servicedeskapi/servicedesk/{serviceDeskId}/requesttype/{requestTypeId}/field";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::CustomerRequestCreateMetaDTO>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_texts("expand", options.expand);
        request.path_number("requestTypeId", self.request_type_id)?;
        request.path_text("serviceDeskId", self.service_desk_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
