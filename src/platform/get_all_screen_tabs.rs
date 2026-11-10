#[derive(Clone, Copy, Debug)]
pub struct GetAllScreenTabs {
    pub screen_id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub project_key: Option<&'options str>,
}

impl crate::Operation for GetAllScreenTabs {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/screens/{screenId}/tabs";

    type Options<'options> = Options<'options>;
    type Reply = crate::List<crate::Object<super::shape::ScreenableTab>>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("projectKey", options.project_key);
        request.path_number("screenId", self.screen_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
