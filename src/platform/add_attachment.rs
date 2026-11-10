#[derive(Clone, Copy, Debug)]
pub struct AddAttachment<'call> {
    pub body: &'call [crate::Part<'call>],
    pub issue_id_or_key: &'call str,
}

impl crate::Operation for AddAttachment<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/api/3/issue/{issueIdOrKey}/attachments";

    type Options<'options> = ();
    type Reply = crate::List<crate::Object<super::shape::Attachment>>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("issueIdOrKey", self.issue_id_or_key)?;
        request.body_multipart(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
