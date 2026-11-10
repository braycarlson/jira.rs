#[derive(Clone, Copy, Debug)]
pub struct DeleteBoard {
    pub board_id: i64,
}

impl crate::Operation for DeleteBoard {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/agile/1.0/board/{boardId}";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("boardId", self.board_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
