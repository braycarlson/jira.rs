#[derive(Clone, Copy, Debug)]
pub struct UpdateScreenScheme<'call> {
    pub body: &'call serde_json::Value,
    pub screen_scheme_id: &'call str,
}

impl crate::Operation for UpdateScreenScheme<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
    const PATH: &'static str = "/rest/api/3/screenscheme/{screenSchemeId}";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("screenSchemeId", self.screen_scheme_id)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
