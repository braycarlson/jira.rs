#[derive(Clone, Copy, Debug)]
pub struct DynamicModulesResourceGetModulesGet;

impl crate::Operation for DynamicModulesResourceGetModulesGet {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/atlassian-connect/1/app/module/dynamic";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::ConnectModules>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
