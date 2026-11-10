#[derive(Clone, Copy, Debug)]
pub struct SetWorklogProperty<'call> {
    pub body: &'call serde_json::Value,
    pub issue_id_or_key: &'call str,
    pub property_key: &'call str,
    pub worklog_id: &'call str,
}

impl crate::Operation for SetWorklogProperty<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;

    const PATH: &'static str =
        "/rest/api/3/issue/{issueIdOrKey}/worklog/{worklogId}/properties/{propertyKey}";

    type Options<'options> = ();
    type Reply = crate::Any;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("issueIdOrKey", self.issue_id_or_key)?;
        request.path_text("propertyKey", self.property_key)?;
        request.path_text("worklogId", self.worklog_id)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
