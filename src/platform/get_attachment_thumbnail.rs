#[derive(Clone, Copy, Debug)]
pub struct GetAttachmentThumbnail<'call> {
    pub id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub fallback_to_default: Option<bool>,
    pub height: Option<i64>,
    pub redirect: Option<bool>,
    pub width: Option<i64>,
}

impl crate::Operation for GetAttachmentThumbnail<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/attachment/thumbnail/{id}";

    type Options<'options> = Options;
    type Reply = crate::Binary;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_boolean("fallbackToDefault", options.fallback_to_default);
        request.query_number("height", options.height);
        request.path_text("id", self.id)?;
        request.query_boolean("redirect", options.redirect);
        request.query_number("width", options.width);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
