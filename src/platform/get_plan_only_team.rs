#[derive(Clone, Copy, Debug)]
pub struct GetPlanOnlyTeam {
    pub plan_id: i64,
    pub plan_only_team_id: i64,
}

impl crate::Operation for GetPlanOnlyTeam {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/plans/plan/{planId}/team/planonly/{planOnlyTeamId}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::GetPlanOnlyTeamResponse>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("planId", self.plan_id)?;
        request.path_number("planOnlyTeamId", self.plan_only_team_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
