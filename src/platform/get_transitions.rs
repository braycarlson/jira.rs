#[derive(Clone, Copy, Debug)]
pub struct GetTransitions<'call> {
    pub issue_id_or_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options str>,
    pub include_unavailable_transitions: Option<bool>,
    pub skip_remote_only_condition: Option<bool>,
    pub sort_by_ops_bar_and_status: Option<bool>,
    pub transition_id: Option<&'options str>,
}

impl crate::Operation for GetTransitions<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/issue/{issueIdOrKey}/transitions";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::Transitions>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("expand", options.expand);

        request.query_boolean(
            "includeUnavailableTransitions",
            options.include_unavailable_transitions,
        );

        request.path_text("issueIdOrKey", self.issue_id_or_key)?;
        request.query_boolean("skipRemoteOnlyCondition", options.skip_remote_only_condition);
        request.query_boolean("sortByOpsBarAndStatus", options.sort_by_ops_bar_and_status);
        request.query_text("transitionId", options.transition_id);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
