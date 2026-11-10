#[derive(Clone, Copy, Debug)]
pub struct GetIssueFieldOption<'call> {
    pub field_key: &'call str,
    pub option_id: i64,
}

impl crate::Operation for GetIssueFieldOption<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/field/{fieldKey}/option/{optionId}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::IssueFieldOption>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("fieldKey", self.field_key)?;
        request.path_number("optionId", self.option_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
