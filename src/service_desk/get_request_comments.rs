#[derive(Clone, Copy, Debug)]
pub struct GetRequestComments<'call> {
    pub issue_id_or_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options [&'options str]>,
    pub internal: Option<bool>,
    pub limit: Option<i64>,
    pub public: Option<bool>,
    pub start: Option<i64>,
}

impl crate::Operation for GetRequestComments<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/servicedeskapi/request/{issueIdOrKey}/comment";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PagedDTOCommentDTO>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_texts("expand", options.expand);
        request.query_boolean("internal", options.internal);
        request.path_text("issueIdOrKey", self.issue_id_or_key)?;
        request.query_number("limit", options.limit);
        request.query_boolean("public", options.public);
        request.query_number("start", options.start);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetRequestComments<'_> {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "start", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::CommentDTO>;
}
