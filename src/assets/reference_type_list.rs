#[derive(Clone, Copy, Debug)]
pub struct ReferenceTypeList<'call> {
    pub workspace_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub include_all: Option<bool>,
    pub object_schema_id: Option<&'options str>,
}

impl crate::Operation for ReferenceTypeList<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/jsm/assets/workspace/{workspaceId}/v1/config/referencetype";

    type Options<'options> = Options<'options>;
    type Reply = crate::List<crate::Object<super::shape::ReferenceType>>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_boolean("includeAll", options.include_all);
        request.query_text("objectSchemaId", options.object_schema_id);
        request.path_text("workspaceId", self.workspace_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
