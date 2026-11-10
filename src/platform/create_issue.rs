#[derive(Clone, Copy, Debug)]
pub struct CreateIssue<'call> {
    pub body: &'call serde_json::Value,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub update_history: Option<bool>,
}

impl crate::Operation for CreateIssue<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/api/3/issue";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::CreatedIssue>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_boolean("updateHistory", options.update_history);
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
