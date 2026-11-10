#[derive(Clone, Copy, Debug)]
pub struct UpdateGadget<'call> {
    pub body: &'call serde_json::Value,
    pub dashboard_id: i64,
    pub gadget_id: i64,
}

impl crate::Operation for UpdateGadget<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
    const PATH: &'static str = "/rest/api/3/dashboard/{dashboardId}/gadget/{gadgetId}";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("dashboardId", self.dashboard_id)?;
        request.path_number("gadgetId", self.gadget_id)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
