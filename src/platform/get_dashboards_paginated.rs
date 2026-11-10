#[derive(Clone, Copy, Debug)]
pub struct GetDashboardsPaginated;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub account_id: Option<&'options str>,
    pub dashboard_name: Option<&'options str>,
    pub expand: Option<&'options str>,
    pub group_id: Option<&'options str>,
    pub groupname: Option<&'options str>,
    pub order_by: Option<&'options str>,
    pub owner: Option<&'options str>,
    pub project_id: Option<i64>,
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
    pub status: Option<&'options str>,
}

impl crate::Operation for GetDashboardsPaginated {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/dashboard/search";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PageBeanDashboard>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("accountId", options.account_id);
        request.query_text("dashboardName", options.dashboard_name);
        request.query_text("expand", options.expand);
        request.query_text("groupId", options.group_id);
        request.query_text("groupname", options.groupname);
        request.query_text("orderBy", options.order_by);
        request.query_text("owner", options.owner);
        request.query_number("projectId", options.project_id);
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);
        request.query_text("status", options.status);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetDashboardsPaginated {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::Dashboard>;
}
