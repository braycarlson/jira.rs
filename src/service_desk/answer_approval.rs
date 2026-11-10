#[derive(Clone, Copy, Debug)]
pub struct AnswerApproval<'call> {
    pub approval_id: i64,
    pub body: &'call serde_json::Value,
    pub issue_id_or_key: &'call str,
}

impl crate::Operation for AnswerApproval<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/servicedeskapi/request/{issueIdOrKey}/approval/{approvalId}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::ApprovalDTO>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("approvalId", self.approval_id)?;
        request.path_text("issueIdOrKey", self.issue_id_or_key)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
