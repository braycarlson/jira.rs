#[derive(Clone, Copy, Debug)]
pub struct CreatePlanOnlyTeam<'call> {
    pub body: &'call serde_json::Value,
    pub plan_id: i64,
}

impl crate::Operation for CreatePlanOnlyTeam<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/api/3/plans/plan/{planId}/team/planonly";

    type Options<'options> = ();
    type Reply = crate::Integer;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("planId", self.plan_id)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
