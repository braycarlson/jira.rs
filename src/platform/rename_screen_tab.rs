#[derive(Clone, Copy, Debug)]
pub struct RenameScreenTab<'call> {
    pub body: &'call serde_json::Value,
    pub screen_id: i64,
    pub tab_id: i64,
}

impl crate::Operation for RenameScreenTab<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
    const PATH: &'static str = "/rest/api/3/screens/{screenId}/tabs/{tabId}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::ScreenableTab>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("screenId", self.screen_id)?;
        request.path_number("tabId", self.tab_id)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
