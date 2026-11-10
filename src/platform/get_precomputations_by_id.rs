#[derive(Clone, Copy, Debug)]
pub struct GetPrecomputationsByID<'call> {
    pub body: &'call serde_json::Value,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub order_by: Option<&'options str>,
}

impl crate::Operation for GetPrecomputationsByID<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/api/3/jql/function/computation/search";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::JQLFunctionPrecomputationGetByIDResponse>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("orderBy", options.order_by);
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
