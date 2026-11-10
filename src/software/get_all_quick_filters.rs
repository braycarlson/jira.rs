#[derive(Clone, Copy, Debug)]
pub struct GetAllQuickFilters {
    pub board_id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
}

impl crate::Operation for GetAllQuickFilters {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/agile/1.0/board/{boardId}/quickfilter";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::GetAllQuickFiltersReply>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("boardId", self.board_id)?;
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetAllQuickFilters {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::GetAllQuickFiltersItem>;
}
