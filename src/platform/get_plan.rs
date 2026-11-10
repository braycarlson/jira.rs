#[derive(Clone, Copy, Debug)]
pub struct GetPlan {
    pub plan_id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub use_group_id: Option<bool>,
}

impl crate::Operation for GetPlan {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/plans/plan/{planId}";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::GetPlanResponse>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("planId", self.plan_id)?;
        request.query_boolean("useGroupId", options.use_group_id);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
