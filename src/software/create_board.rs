#[derive(Clone, Copy, Debug)]
pub struct CreateBoard<'call> {
    pub body: &'call serde_json::Value,
}

impl crate::Operation for CreateBoard<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/agile/1.0/board";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::CreateBoardReply>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
