#[derive(Clone, Copy, Debug)]
pub struct GetRequestTypePropertiesKeys<'call> {
    pub request_type_id: i64,
    pub service_desk_id: &'call str,
}

impl crate::Operation for GetRequestTypePropertiesKeys<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;

    const PATH: &'static str =
        "/rest/servicedeskapi/servicedesk/{serviceDeskId}/requesttype/{requestTypeId}/property";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::PropertyKeys>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("requestTypeId", self.request_type_id)?;
        request.path_text("serviceDeskId", self.service_desk_id)?;
        request.header("X-ExperimentalApi", "opt-in");

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
