#[derive(Clone, Copy, Debug)]
pub struct UpdateCustomFieldValue<'call> {
    pub body: &'call serde_json::Value,
    pub field_id_or_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub generate_app_events: Option<bool>,
    pub generate_changelog: Option<bool>,
}

impl crate::Operation for UpdateCustomFieldValue<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
    const PATH: &'static str = "/rest/api/3/app/field/{fieldIdOrKey}/value";

    type Options<'options> = Options;
    type Reply = crate::Nothing;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("fieldIdOrKey", self.field_id_or_key)?;
        request.query_boolean("generateAppEvents", options.generate_app_events);
        request.query_boolean("generateChangelog", options.generate_changelog);
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
