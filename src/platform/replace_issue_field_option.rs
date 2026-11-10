#[derive(Clone, Copy, Debug)]
pub struct ReplaceIssueFieldOption<'call> {
    pub field_key: &'call str,
    pub option_id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub jql: Option<&'options str>,
    pub override_editable_flag: Option<bool>,
    pub override_screen_security: Option<bool>,
    pub replace_with: Option<i64>,
}

impl crate::Operation for ReplaceIssueFieldOption<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/api/3/field/{fieldKey}/option/{optionId}/issue";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::TaskProgressBeanRemoveOptionFromIssuesResult>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("fieldKey", self.field_key)?;
        request.query_text("jql", options.jql);
        request.path_number("optionId", self.option_id)?;
        request.query_boolean("overrideEditableFlag", options.override_editable_flag);
        request.query_boolean("overrideScreenSecurity", options.override_screen_security);
        request.query_number("replaceWith", options.replace_with);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
