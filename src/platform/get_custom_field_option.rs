#[derive(Clone, Copy, Debug)]
pub struct GetCustomFieldOption<'call> {
    pub id: &'call str,
}

impl crate::Operation for GetCustomFieldOption<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/customFieldOption/{id}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::CustomFieldOption>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("id", self.id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
