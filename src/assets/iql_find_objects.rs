#[derive(Clone, Copy, Debug)]
pub struct IQLFindObjects<'call> {
    pub workspace_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub include_attributes: Option<bool>,
    pub include_attributes_deep: Option<i64>,
    pub include_extended_information: Option<bool>,
    pub include_type_attributes: Option<bool>,
    pub iql: Option<&'options str>,
    pub page: Option<i64>,
    pub result_per_page: Option<i64>,
}

impl crate::Operation for IQLFindObjects<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/jsm/assets/workspace/{workspaceId}/v1/iql/objects";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::ObjectListResult>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_boolean("includeAttributes", options.include_attributes);
        request.query_number("includeAttributesDeep", options.include_attributes_deep);
        request.query_boolean("includeExtendedInfo", options.include_extended_information);
        request.query_boolean("includeTypeAttributes", options.include_type_attributes);
        request.query_text("iql", options.iql);
        request.query_number("page", options.page);
        request.query_number("resultPerPage", options.result_per_page);
        request.path_text("workspaceId", self.workspace_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for IQLFindObjects<'_> {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "page", items: Some("/objectEntries"), next: crate::Next::Number };

    type Item = crate::Object<super::shape::Object>;
}
