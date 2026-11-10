#[derive(Clone, Copy, Debug)]
pub struct GetProjectRules<'call> {
    pub project_id_or_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub rule_type_ids: Option<&'options [&'options str]>,
}

impl crate::Operation for GetProjectRules<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/policies/project/{projectIdOrKey}/rules";

    type Options<'options> = Options<'options>;
    type Reply = crate::List<crate::Object<super::shape::StatePolicyRule>>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("projectIdOrKey", self.project_id_or_key)?;
        request.query_texts("ruleTypeIds", options.rule_type_ids);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
