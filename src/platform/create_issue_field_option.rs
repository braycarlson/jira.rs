#[derive(Clone, Copy, Debug)]
pub struct CreateIssueFieldOption<'call> {
    pub body: &'call serde_json::Value,
    pub field_key: &'call str,
}

impl crate::Operation for CreateIssueFieldOption<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/api/3/field/{fieldKey}/option";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::IssueFieldOption>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("fieldKey", self.field_key)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
