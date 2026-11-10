#[derive(Clone, Copy, Debug)]
pub struct GetAvatarImageByOwner<'call> {
    pub entity_id: &'call str,
    pub kind: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub format: Option<&'options str>,
    pub size: Option<&'options str>,
}

impl crate::Operation for GetAvatarImageByOwner<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/universal_avatar/view/type/{type}/owner/{entityId}";

    type Options<'options> = Options<'options>;
    type Reply = crate::Binary;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("entityId", self.entity_id)?;
        request.query_text("format", options.format);
        request.path_text("type", self.kind)?;
        request.query_text("size", options.size);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
