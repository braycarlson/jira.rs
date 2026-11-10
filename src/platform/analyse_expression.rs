#[derive(Clone, Copy, Debug)]
pub struct AnalyseExpression<'call> {
    pub body: &'call serde_json::Value,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub check: Option<&'options str>,
}

impl crate::Operation for AnalyseExpression<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/api/3/expression/analyse";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::JiraExpressionsAnalysis>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("check", options.check);
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
