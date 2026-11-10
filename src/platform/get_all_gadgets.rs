#[derive(Clone, Copy, Debug)]
pub struct GetAllGadgets {
    pub dashboard_id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub gadget_id: Option<&'options [i64]>,
    pub module_key: Option<&'options [&'options str]>,
    pub uri: Option<&'options [&'options str]>,
}

impl crate::Operation for GetAllGadgets {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/dashboard/{dashboardId}/gadget";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::DashboardGadgetResponse>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("dashboardId", self.dashboard_id)?;
        request.query_numbers("gadgetId", options.gadget_id);
        request.query_texts("moduleKey", options.module_key);
        request.query_texts("uri", options.uri);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
