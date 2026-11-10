#[derive(Clone, Copy, Debug)]
pub struct RemoveTemplate<'call> {
    pub template_key: &'call str,
}

impl crate::Operation for RemoveTemplate<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/api/3/project-template/remove-template";

    type Options<'options> = ();
    type Reply = crate::Any;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("templateKey", Some(self.template_key));

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
