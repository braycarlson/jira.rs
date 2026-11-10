#[derive(Clone, Copy, Debug)]
pub struct SchemaFindAllObjectTypesFlat<'call> {
    pub id: &'call str,
    pub workspace_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub exclude: Option<&'options str>,
    pub include_object_counts: Option<bool>,
    pub query: Option<&'options str>,
}

impl crate::Operation for SchemaFindAllObjectTypesFlat<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;

    const PATH: &'static str =
        "/jsm/assets/workspace/{workspaceId}/v1/objectschema/{id}/objecttypes/flat";

    type Options<'options> = Options<'options>;
    type Reply = crate::List<crate::Object<super::shape::ObjectType>>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("exclude", options.exclude);
        request.path_text("id", self.id)?;
        request.query_boolean("includeObjectCounts", options.include_object_counts);
        request.query_text("query", options.query);
        request.path_text("workspaceId", self.workspace_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
