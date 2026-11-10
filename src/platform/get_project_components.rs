#[derive(Clone, Copy, Debug)]
pub struct GetProjectComponents<'call> {
    pub project_id_or_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub component_source: Option<&'options str>,
}

impl crate::Operation for GetProjectComponents<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/project/{projectIdOrKey}/components";

    type Options<'options> = Options<'options>;
    type Reply = crate::List<crate::Object<super::shape::ProjectComponent>>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("componentSource", options.component_source);
        request.path_text("projectIdOrKey", self.project_id_or_key)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
