#[derive(Clone, Copy, Debug)]
pub struct SetRequestTypeProperty<'call> {
    pub body: &'call serde_json::Value,
    pub property_key: &'call str,
    pub request_type_id: i64,
    pub service_desk_id: &'call str,
}

impl crate::Operation for SetRequestTypeProperty<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;

    const PATH: &'static str = concat!(
        "/rest/servicedeskapi/servicedesk/{serviceDeskId}/requesttype/{requestTypeId}/property/",
        "{propertyKey}",
    );

    type Options<'options> = ();
    type Reply = crate::Any;

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
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
