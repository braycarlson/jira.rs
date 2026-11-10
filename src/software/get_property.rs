#[derive(Clone, Copy, Debug)]
pub struct GetProperty<'call> {
    pub property_key: &'call str,
    pub sprint_id: &'call str,
}

impl crate::Operation for GetProperty<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
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

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
