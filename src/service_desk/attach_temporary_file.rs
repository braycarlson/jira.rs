#[derive(Clone, Copy, Debug)]
pub struct AttachTemporaryFile<'call> {
    pub body: &'call [crate::Part<'call>],
    pub service_desk_id: &'call str,
}

impl crate::Operation for AttachTemporaryFile<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;

    const PATH: &'static str =
        "/rest/servicedeskapi/servicedesk/{serviceDeskId}/attachTemporaryFile";

    type Options<'options> = ();
    type Reply = crate::Any;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("serviceDeskId", self.service_desk_id)?;
        request.body_multipart(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
