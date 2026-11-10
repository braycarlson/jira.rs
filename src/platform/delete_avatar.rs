#[derive(Clone, Copy, Debug)]
pub struct DeleteAvatar<'call> {
    pub id: i64,
    pub kind: &'call str,
    pub owning_object_id: &'call str,
}

impl crate::Operation for DeleteAvatar<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;

    const PATH: &'static str =
        "/rest/api/3/universal_avatar/type/{type}/owner/{owningObjectId}/avatar/{id}";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("id", self.id)?;
        request.path_text("type", self.kind)?;
        request.path_text("owningObjectId", self.owning_object_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
