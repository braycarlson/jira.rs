#[derive(Clone, Copy, Debug)]
pub struct GetAllSprints {
    pub board_id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
    pub state: Option<&'options str>,
}

impl crate::Operation for GetAllSprints {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/agile/1.0/board/{boardId}/sprint";

    type Options<'options> = Options<'options>;
    type Reply = crate::Any;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("boardId", self.board_id)?;
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);
        request.query_text("state", options.state);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetAllSprints {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Any;
}
