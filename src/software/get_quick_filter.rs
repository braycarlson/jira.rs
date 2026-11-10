#[derive(Clone, Copy, Debug)]
pub struct GetQuickFilter {
    pub board_id: i64,
    pub quick_filter_id: i64,
}

impl crate::Operation for GetQuickFilter {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/agile/1.0/board/{boardId}/quickfilter/{quickFilterId}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::GetQuickFilterReply>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("boardId", self.board_id)?;
        request.path_number("quickFilterId", self.quick_filter_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
