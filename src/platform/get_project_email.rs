#[derive(Clone, Copy, Debug)]
pub struct GetProjectEmail {
    pub project_id: i64,
}

impl crate::Operation for GetProjectEmail {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/project/{projectId}/email";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::ProjectEmailAddress>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("projectId", self.project_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
