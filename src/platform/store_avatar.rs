#[derive(Clone, Copy, Debug)]
pub struct StoreAvatar<'call> {
    pub body: crate::Upload<'call>,
    pub entity_id: &'call str,
    pub kind: &'call str,
    pub size: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub x_value: Option<i64>,
    pub y_value: Option<i64>,
}

impl crate::Operation for StoreAvatar<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/api/3/universal_avatar/type/{type}/owner/{entityId}";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::Avatar>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("entityId", self.entity_id)?;
        request.path_text("type", self.kind)?;
        request.query_number("size", Some(self.size));
        request.query_number("x", options.x_value);
        request.query_number("y", options.y_value);
        request.header("X-Atlassian-Token", "no-check");
        request.body_upload(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
