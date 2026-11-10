#[derive(Clone, Copy, Debug)]
pub struct GetTeams {
    pub plan_id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub cursor: Option<&'options str>,
    pub results_max: Option<i64>,
}

impl crate::Operation for GetTeams {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/plans/plan/{planId}/team";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PageWithCursorGetTeamResponseForPage>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("cursor", options.cursor);
        request.path_number("planId", self.plan_id)?;
        request.query_number("maxResults", options.results_max);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetTeams {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "cursor",
        items: Some("/values"),
        next: crate::Next::Token("/nextPageCursor"),
    };

    type Item = crate::Object<super::shape::GetTeamResponseForPage>;
}
