#[derive(Clone, Copy, Debug)]
pub struct SetProjectProperty<'call> {
    pub body: &'call serde_json::Value,
    pub project_id_or_key: &'call str,
    pub property_key: &'call str,
}

impl crate::Operation for SetProjectProperty<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
    const PATH: &'static str = "/rest/api/3/project/{projectIdOrKey}/properties/{propertyKey}";

    type Options<'options> = ();
    type Reply = crate::Any;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("projectIdOrKey", self.project_id_or_key)?;
        request.path_text("propertyKey", self.property_key)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
