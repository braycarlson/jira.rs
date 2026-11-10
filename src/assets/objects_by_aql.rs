#[derive(Clone, Copy, Debug)]
pub struct ObjectsByAQL<'call> {
    pub body: &'call serde_json::Value,
    pub workspace_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub include_attributes: Option<bool>,
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
}

impl crate::Operation for ObjectsByAQL<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/jsm/assets/workspace/{workspaceId}/v1/object/aql";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::ObjectListInclTypeAttributesEntryResult>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_boolean("includeAttributes", options.include_attributes);
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);
        request.path_text("workspaceId", self.workspace_id)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
