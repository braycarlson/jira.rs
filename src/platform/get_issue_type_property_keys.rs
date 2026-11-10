#[derive(Clone, Copy, Debug)]
pub struct GetIssueTypePropertyKeys<'call> {
    pub issue_type_id: &'call str,
}

impl crate::Operation for GetIssueTypePropertyKeys<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/issuetype/{issueTypeId}/properties";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::PropertyKeys>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("issueTypeId", self.issue_type_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
