#[derive(Clone, Copy, Debug)]
pub struct AddScreenTabField<'call> {
    pub body: &'call serde_json::Value,
    pub screen_id: i64,
    pub tab_id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub skip_field_association: Option<bool>,
}

impl crate::Operation for AddScreenTabField<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/api/3/screens/{screenId}/tabs/{tabId}/fields";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::ScreenableField>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("screenId", self.screen_id)?;
        request.query_boolean("skipFieldAssociation", options.skip_field_association);
        request.path_number("tabId", self.tab_id)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
