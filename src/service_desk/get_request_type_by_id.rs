#[derive(Clone, Copy, Debug)]
pub struct GetRequestTypeByID<'call> {
    pub request_type_id: &'call str,
    pub service_desk_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options [&'options str]>,
}

impl crate::Operation for GetRequestTypeByID<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;

    const PATH: &'static str =
        "/rest/servicedeskapi/servicedesk/{serviceDeskId}/requesttype/{requestTypeId}";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::RequestTypeDTO>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_texts("expand", options.expand);
        request.path_text("requestTypeId", self.request_type_id)?;
        request.path_text("serviceDeskId", self.service_desk_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
