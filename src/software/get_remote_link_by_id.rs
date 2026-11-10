#[derive(Clone, Copy, Debug)]
pub struct GetRemoteLinkByID<'call> {
    pub remote_link_id: &'call str,
}

impl crate::Operation for GetRemoteLinkByID<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/remotelinks/1.0/remotelink/{remoteLinkId}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::GetRemoteLinkByIDReply>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("remoteLinkId", self.remote_link_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
