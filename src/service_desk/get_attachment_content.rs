#[derive(Clone, Copy, Debug)]
pub struct GetAttachmentContent<'call> {
    pub attachment_id: i64,
    pub issue_id_or_key: &'call str,
}

impl crate::Operation for GetAttachmentContent<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;

    const PATH: &'static str =
        "/rest/servicedeskapi/request/{issueIdOrKey}/attachment/{attachmentId}";

    type Options<'options> = ();
    type Reply = crate::Binary;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("attachmentId", self.attachment_id)?;
        request.path_text("issueIdOrKey", self.issue_id_or_key)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
