#[derive(Clone, Copy, Debug)]
pub struct GetWorkflowTransitionRuleConfigurations<'call> {
    pub types: &'call [&'call str],
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub draft: Option<bool>,
    pub expand: Option<&'options str>,
    pub keys: Option<&'options [&'options str]>,
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
    pub with_tags: Option<&'options [&'options str]>,
    pub workflow_names: Option<&'options [&'options str]>,
}

impl crate::Operation for GetWorkflowTransitionRuleConfigurations<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/workflow/rule/config";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PageBeanWorkflowTransitionRules>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_boolean("draft", options.draft);
        request.query_text("expand", options.expand);
        request.query_texts("keys", options.keys);
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);
        request.query_texts("types", Some(self.types));
        request.query_texts("withTags", options.with_tags);
        request.query_texts("workflowNames", options.workflow_names);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetWorkflowTransitionRuleConfigurations<'_> {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::WorkflowTransitionRules>;
}
