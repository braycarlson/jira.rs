#[derive(Clone, Copy, Debug)]
pub struct GetWorkspaces;

impl crate::Operation for GetWorkspaces {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/operations/1.0/linkedWorkspaces";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::GetWorkspacesReply>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
