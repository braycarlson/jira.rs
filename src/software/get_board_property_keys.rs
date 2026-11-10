#[derive(Clone, Copy, Debug)]
pub struct GetBoardPropertyKeys<'call> {
    pub board_id: &'call str,
}

impl crate::Operation for GetBoardPropertyKeys<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/agile/1.0/board/{boardId}/properties";

    type Options<'options> = ();
    type Reply = crate::Any;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("boardId", self.board_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
