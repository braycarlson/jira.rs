#[derive(Clone, Copy, Debug)]
pub struct GetAvatars<'call> {
    pub entity_id: &'call str,
    pub kind: &'call str,
}

impl crate::Operation for GetAvatars<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/universal_avatar/type/{type}/owner/{entityId}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::Avatars>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("entityId", self.entity_id)?;
        request.path_text("type", self.kind)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
