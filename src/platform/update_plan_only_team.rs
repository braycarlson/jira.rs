#[derive(Clone, Copy, Debug)]
pub struct UpdatePlanOnlyTeam<'call> {
    pub body: &'call serde_json::Value,
    pub plan_id: i64,
    pub plan_only_team_id: i64,
}

impl crate::Operation for UpdatePlanOnlyTeam<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
    const PATH: &'static str = "/rest/api/3/plans/plan/{planId}/team/planonly/{planOnlyTeamId}";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("planId", self.plan_id)?;
        request.path_number("planOnlyTeamId", self.plan_only_team_id)?;
        request.body_json_patch(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
