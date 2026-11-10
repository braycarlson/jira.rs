#[derive(Clone, Copy, Debug)]
pub struct GetAtlassianTeam<'call> {
    pub atlassian_team_id: &'call str,
    pub plan_id: i64,
}

impl crate::Operation for GetAtlassianTeam<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/plans/plan/{planId}/team/atlassian/{atlassianTeamId}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::GetAtlassianTeamResponse>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("atlassianTeamId", self.atlassian_team_id)?;
        request.path_number("planId", self.plan_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
