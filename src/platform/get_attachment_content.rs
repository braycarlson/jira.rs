#[derive(Clone, Copy, Debug)]
pub struct GetAttachmentContent<'call> {
    pub id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub redirect: Option<bool>,
}

impl crate::Operation for GetAttachmentContent<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/attachment/content/{id}";

    type Options<'options> = Options;
    type Reply = crate::Binary;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("id", self.id)?;
        request.query_boolean("redirect", options.redirect);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
