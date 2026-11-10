#[derive(Clone, Copy, Debug)]
pub struct DynamicModulesResourceRemoveModulesDelete;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub module_key: Option<&'options [&'options str]>,
}

impl crate::Operation for DynamicModulesResourceRemoveModulesDelete {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/atlassian-connect/1/app/module/dynamic";

    type Options<'options> = Options<'options>;
    type Reply = crate::Nothing;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_texts("moduleKey", options.module_key);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
