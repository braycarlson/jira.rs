#[derive(Clone, Copy, Debug)]
pub struct IconLoadImage<'call> {
    pub id: &'call str,
    pub workspace_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub size: Option<i64>,
}

impl crate::Operation for IconLoadImage<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/jsm/assets/workspace/{workspaceId}/v1/icon/{id}/icon.png";

    type Options<'options> = Options;
    type Reply = crate::Binary;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("id", self.id)?;
        request.query_number("size", options.size);
        request.path_text("workspaceId", self.workspace_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
