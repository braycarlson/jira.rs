#[derive(Clone, Copy, Debug)]
pub struct GetIssuePickerResource;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub issue_key_current: Option<&'options str>,
    pub jql_current: Option<&'options str>,
    pub project_id_current: Option<&'options str>,
    pub query: Option<&'options str>,
    pub show_sub_task_parent: Option<bool>,
    pub show_sub_tasks: Option<bool>,
}

impl crate::Operation for GetIssuePickerResource {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/issue/picker";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::IssuePickerSuggestions>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("currentIssueKey", options.issue_key_current);
        request.query_text("currentJQL", options.jql_current);
        request.query_text("currentProjectId", options.project_id_current);
        request.query_text("query", options.query);
        request.query_boolean("showSubTaskParent", options.show_sub_task_parent);
        request.query_boolean("showSubTasks", options.show_sub_tasks);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
