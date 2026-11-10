#[derive(Clone, Copy, Debug)]
pub struct GetAttachmentsForRequest<'call> {
    pub issue_id_or_key: &'call str,
    pub limit: i64,
    pub start: i64,
}

impl crate::Operation for GetAttachmentsForRequest<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/servicedeskapi/request/{issueIdOrKey}/attachment";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::PagedDTOAttachmentDTO>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("issueIdOrKey", self.issue_id_or_key)?;
        request.query_number("limit", Some(self.limit));
        request.query_number("start", Some(self.start));

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetAttachmentsForRequest<'_> {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "start", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::AttachmentDTO>;
}
