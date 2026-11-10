#[derive(Clone, Copy, Debug)]
pub struct ReplaceCustomFieldOption<'call> {
    pub context_id: i64,
    pub field_id: &'call str,
    pub option_id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub jql: Option<&'options str>,
    pub replace_with: Option<i64>,
}

impl crate::Operation for ReplaceCustomFieldOption<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;

    const PATH: &'static str =
        "/rest/api/3/field/{fieldId}/context/{contextId}/option/{optionId}/issue";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::TaskProgressBeanRemoveOptionFromIssuesResult>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("contextId", self.context_id)?;
        request.path_text("fieldId", self.field_id)?;
        request.query_text("jql", options.jql);
        request.path_number("optionId", self.option_id)?;
        request.query_number("replaceWith", options.replace_with);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
