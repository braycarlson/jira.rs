#[derive(Clone, Copy, Debug)]
pub struct DeleteCustomField<'call> {
    pub id: &'call str,
}

impl crate::Operation for DeleteCustomField<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/api/3/field/{id}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::TaskProgressBeanObject>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("id", self.id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
