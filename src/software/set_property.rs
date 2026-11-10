#[derive(Clone, Copy, Debug)]
pub struct SetProperty<'call> {
    pub body: &'call serde_json::Value,
    pub property_key: &'call str,
    pub sprint_id: &'call str,
}

impl crate::Operation for SetProperty<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
    const PATH: &'static str = "/rest/agile/1.0/sprint/{sprintId}/properties/{propertyKey}";

    type Options<'options> = ();
    type Reply = crate::Any;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("propertyKey", self.property_key)?;
        request.path_text("sprintId", self.sprint_id)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
