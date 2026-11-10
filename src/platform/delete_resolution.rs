#[derive(Clone, Copy, Debug)]
pub struct DeleteResolution<'call> {
    pub id: &'call str,
    pub replace_with: &'call str,
}

impl crate::Operation for DeleteResolution<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/api/3/resolution/{id}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::TaskProgressBeanObject>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("id", self.id)?;
        request.query_text("replaceWith", Some(self.replace_with));

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
