#[derive(Clone, Copy, Debug)]
pub struct DeleteDashboardItemProperty<'call> {
    pub dashboard_id: &'call str,
    pub item_id: &'call str,
    pub property_key: &'call str,
}

impl crate::Operation for DeleteDashboardItemProperty<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;

    const PATH: &'static str =
        "/rest/api/3/dashboard/{dashboardId}/items/{itemId}/properties/{propertyKey}";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("dashboardId", self.dashboard_id)?;
        request.path_text("itemId", self.item_id)?;
        request.path_text("propertyKey", self.property_key)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
