#[derive(Clone, Copy, Debug)]
pub struct GetApplicationRole<'call> {
    pub key: &'call str,
}

impl crate::Operation for GetApplicationRole<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/applicationrole/{key}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::ApplicationRole>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("key", self.key)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
