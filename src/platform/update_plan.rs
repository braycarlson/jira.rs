#[derive(Clone, Copy, Debug)]
pub struct UpdatePlan<'call> {
    pub body: &'call serde_json::Value,
    pub plan_id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub use_group_id: Option<bool>,
}

impl crate::Operation for UpdatePlan<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
    const PATH: &'static str = "/rest/api/3/plans/plan/{planId}";

    type Options<'options> = Options;
    type Reply = crate::Nothing;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("planId", self.plan_id)?;
        request.query_boolean("useGroupId", options.use_group_id);
        request.body_json_patch(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
