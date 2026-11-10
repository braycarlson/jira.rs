#[derive(Clone, Copy, Debug)]
pub struct MoveIssuesToBoard<'call> {
    pub board_id: i64,
    pub body: &'call serde_json::Value,
}

impl crate::Operation for MoveIssuesToBoard<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/agile/1.0/board/{boardId}/issue";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::MoveIssuesToBoardReply>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("boardId", self.board_id)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
