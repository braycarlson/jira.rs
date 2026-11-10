#[derive(Clone, Copy, Debug)]
pub struct UpdateMultipleCustomFieldValues<'call> {
    pub body: &'call serde_json::Value,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub generate_app_events: Option<bool>,
    pub generate_changelog: Option<bool>,
}

impl crate::Operation for UpdateMultipleCustomFieldValues<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/api/3/app/field/value";

    type Options<'options> = Options;
    type Reply = crate::Nothing;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_boolean("generateAppEvents", options.generate_app_events);
        request.query_boolean("generateChangelog", options.generate_changelog);
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
