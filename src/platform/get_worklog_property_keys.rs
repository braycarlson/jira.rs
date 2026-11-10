#[derive(Clone, Copy, Debug)]
pub struct GetWorklogPropertyKeys<'call> {
    pub issue_id_or_key: &'call str,
    pub worklog_id: &'call str,
}

impl crate::Operation for GetWorklogPropertyKeys<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/issue/{issueIdOrKey}/worklog/{worklogId}/properties";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::PropertyKeys>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("issueIdOrKey", self.issue_id_or_key)?;
        request.path_text("worklogId", self.worklog_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
