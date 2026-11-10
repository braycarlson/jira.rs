#[derive(Clone, Copy, Debug)]
pub struct GetCommentAttachments<'call> {
    pub comment_id: i64,
    pub issue_id_or_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub limit: Option<i64>,
    pub start: Option<i64>,
}

impl crate::Operation for GetCommentAttachments<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;

    const PATH: &'static str =
        "/rest/servicedeskapi/request/{issueIdOrKey}/comment/{commentId}/attachment";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::PagedDTOAttachmentDTO>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("commentId", self.comment_id)?;
        request.path_text("issueIdOrKey", self.issue_id_or_key)?;
        request.query_number("limit", options.limit);
        request.query_number("start", options.start);
        request.header("X-ExperimentalApi", "opt-in");

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetCommentAttachments<'_> {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "start", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::AttachmentDTO>;
}
