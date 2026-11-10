#[derive(Clone, Copy, Debug)]
pub struct GetIssueProperty<'call> {
    pub issue_id_or_key: &'call str,
    pub property_key: &'call str,
}

impl crate::Operation for GetIssueProperty<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/issue/{issueIdOrKey}/properties/{propertyKey}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::EntityProperty>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("issueIdOrKey", self.issue_id_or_key)?;
        request.path_text("propertyKey", self.property_key)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
