#[derive(Clone, Copy, Debug)]
pub struct GetAllSystemAvatars<'call> {
    pub kind: &'call str,
}

impl crate::Operation for GetAllSystemAvatars<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/avatar/{type}/system";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::SystemAvatars>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("type", self.kind)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
