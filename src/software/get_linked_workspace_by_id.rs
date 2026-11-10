#[derive(Clone, Copy, Debug)]
pub struct GetLinkedWorkspaceByID<'call> {
    pub workspace_id: &'call str,
}

impl crate::Operation for GetLinkedWorkspaceByID<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/security/1.0/linkedWorkspaces/{workspaceId}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::GetLinkedWorkspaceByIDReply>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("workspaceId", self.workspace_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
