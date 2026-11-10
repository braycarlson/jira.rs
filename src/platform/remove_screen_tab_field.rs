#[derive(Clone, Copy, Debug)]
pub struct RemoveScreenTabField<'call> {
    pub id: &'call str,
    pub screen_id: i64,
    pub tab_id: i64,
}

impl crate::Operation for RemoveScreenTabField<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/api/3/screens/{screenId}/tabs/{tabId}/fields/{id}";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("id", self.id)?;
        request.path_number("screenId", self.screen_id)?;
        request.path_number("tabId", self.tab_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
