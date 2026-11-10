#[derive(Clone, Copy, Debug)]
pub struct FindUsersAndGroups<'call> {
    pub query: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub avatar_size: Option<&'options str>,
    pub case_insensitive: Option<bool>,
    pub exclude_connect_addons: Option<bool>,
    pub field_id: Option<&'options str>,
    pub include_ai_agents: Option<bool>,
    pub issue_type_id: Option<&'options [&'options str]>,
    pub project_id: Option<&'options [&'options str]>,
    pub results_max: Option<i64>,
    pub show_avatar: Option<bool>,
}

impl crate::Operation for FindUsersAndGroups<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/groupuserpicker";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::FoundUsersAndGroups>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("avatarSize", options.avatar_size);
        request.query_boolean("caseInsensitive", options.case_insensitive);
        request.query_boolean("excludeConnectAddons", options.exclude_connect_addons);
        request.query_text("fieldId", options.field_id);
        request.query_boolean("includeAiAgents", options.include_ai_agents);
        request.query_texts("issueTypeId", options.issue_type_id);
        request.query_texts("projectId", options.project_id);
        request.query_text("query", Some(self.query));
        request.query_number("maxResults", options.results_max);
        request.query_boolean("showAvatar", options.show_avatar);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
