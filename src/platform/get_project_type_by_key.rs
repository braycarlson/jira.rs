#[derive(Clone, Copy, Debug)]
pub struct GetProjectTypeByKey<'call> {
    pub project_type_key: &'call str,
}

impl crate::Operation for GetProjectTypeByKey<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/project/type/{projectTypeKey}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::ProjectType>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("projectTypeKey", self.project_type_key)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
