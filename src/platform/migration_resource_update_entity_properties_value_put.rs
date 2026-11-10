#[derive(Clone, Copy, Debug)]
pub struct MigrationResourceUpdateEntityPropertiesValuePut<'call> {
    pub atlassian_transfer_id: &'call str,
    pub body: &'call serde_json::Value,
    pub entity_type: &'call str,
}

impl crate::Operation for MigrationResourceUpdateEntityPropertiesValuePut<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
    const PATH: &'static str = "/rest/atlassian-connect/1/migration/properties/{entityType}";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.header("Atlassian-Transfer-Id", self.atlassian_transfer_id);
        request.path_text("entityType", self.entity_type)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
