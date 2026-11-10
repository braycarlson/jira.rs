#[derive(Clone, Copy, Debug)]
pub struct SchemaFindAllAttributes<'call> {
    pub id: &'call str,
    pub workspace_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub extended: Option<bool>,
    pub only_value_editable: Option<bool>,
    pub query: Option<&'options str>,
}

impl crate::Operation for SchemaFindAllAttributes<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;

    const PATH: &'static str =
        "/jsm/assets/workspace/{workspaceId}/v1/objectschema/{id}/attributes";

    type Options<'options> = Options<'options>;
    type Reply = crate::List<crate::Object<super::shape::ObjectTypeAttribute>>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_boolean("extended", options.extended);
        request.path_text("id", self.id)?;
        request.query_boolean("onlyValueEditable", options.only_value_editable);
        request.query_text("query", options.query);
        request.path_text("workspaceId", self.workspace_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
