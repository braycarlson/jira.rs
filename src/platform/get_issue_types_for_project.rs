#[derive(Clone, Copy, Debug)]
pub struct GetIssueTypesForProject {
    pub project_id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub level: Option<i64>,
}

impl crate::Operation for GetIssueTypesForProject {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/issuetype/project";

    type Options<'options> = Options;
    type Reply = crate::List<crate::Object<super::shape::IssueTypeDetails>>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_number("level", options.level);
        request.query_number("projectId", Some(self.project_id));

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
