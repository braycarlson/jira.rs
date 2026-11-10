#[derive(Clone, Copy, Debug)]
pub struct UpdateIssueFieldOption<'call> {
    pub body: &'call serde_json::Value,
    pub field_key: &'call str,
    pub option_id: i64,
}

impl crate::Operation for UpdateIssueFieldOption<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
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
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
