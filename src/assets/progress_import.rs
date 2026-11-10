#[derive(Clone, Copy, Debug)]
pub struct ProgressImport<'call> {
    pub id: &'call str,
    pub workspace_id: &'call str,
}

impl crate::Operation for ProgressImport<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;

    const PATH: &'static str =
        "/jsm/assets/workspace/{workspaceId}/v1/progress/category/imports/{id}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::Progress>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("id", self.id)?;
        request.path_text("workspaceId", self.workspace_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
