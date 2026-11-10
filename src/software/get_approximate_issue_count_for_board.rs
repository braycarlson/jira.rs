#[derive(Clone, Copy, Debug)]
pub struct GetApproximateIssueCountForBoard {
    pub board_id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub jql: Option<&'options str>,
}

impl crate::Operation for GetApproximateIssueCountForBoard {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/software/1.0/board/{boardId}/issue/approximate-count";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::IssueCountBean>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("boardId", self.board_id)?;
        request.query_text("jql", options.jql);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
