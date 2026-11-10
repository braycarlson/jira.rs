#[derive(Clone, Copy, Debug)]
pub struct GetStatusesByName<'call> {
    pub name: &'call [&'call str],
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub project_id: Option<&'options str>,
}

impl crate::Operation for GetStatusesByName<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/statuses/byNames";

    type Options<'options> = Options<'options>;
    type Reply = crate::List<crate::Object<super::shape::JiraStatus>>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_texts("name", Some(self.name));
        request.query_text("projectId", options.project_id);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
