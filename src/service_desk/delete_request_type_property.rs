#[derive(Clone, Copy, Debug)]
pub struct DeleteRequestTypeProperty<'call> {
    pub property_key: &'call str,
    pub request_type_id: i64,
    pub service_desk_id: &'call str,
}

impl crate::Operation for DeleteRequestTypeProperty<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;

    const PATH: &'static str = concat!(
        "/rest/servicedeskapi/servicedesk/{serviceDeskId}/requesttype/{requestTypeId}/property/",
        "{propertyKey}",
    );

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("propertyKey", self.property_key)?;
        request.path_number("requestTypeId", self.request_type_id)?;
        request.path_text("serviceDeskId", self.service_desk_id)?;
        request.header("X-ExperimentalApi", "opt-in");

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
