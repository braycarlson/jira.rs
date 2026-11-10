#[derive(Clone, Copy, Debug)]
pub struct DeleteCustomFieldContext<'call> {
    pub context_id: i64,
    pub field_id: &'call str,
}

impl crate::Operation for DeleteCustomFieldContext<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/api/3/field/{fieldId}/context/{contextId}";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("contextId", self.context_id)?;
        request.path_text("fieldId", self.field_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
