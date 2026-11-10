#[derive(Clone, Copy, Debug)]
pub struct CreatePlan<'call> {
    pub body: &'call serde_json::Value,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub use_group_id: Option<bool>,
}

impl crate::Operation for CreatePlan<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/api/3/plans/plan";

    type Options<'options> = Options;
    type Reply = crate::Integer;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_boolean("useGroupId", options.use_group_id);
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
