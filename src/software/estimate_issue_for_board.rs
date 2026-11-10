#[derive(Clone, Copy, Debug)]
pub struct EstimateIssueForBoard<'call> {
    pub body: &'call serde_json::Value,
    pub issue_id_or_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub board_id: Option<i64>,
}

impl crate::Operation for EstimateIssueForBoard<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
    const PATH: &'static str = "/rest/agile/1.0/issue/{issueIdOrKey}/estimation";

    type Options<'options> = Options;
    type Reply = crate::Any;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_number("boardId", options.board_id);
        request.path_text("issueIdOrKey", self.issue_id_or_key)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
