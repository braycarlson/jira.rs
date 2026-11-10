#[derive(Clone, Copy, Debug)]
pub struct DeleteIssueTypeScheme {
    pub issue_type_scheme_id: i64,
}

impl crate::Operation for DeleteIssueTypeScheme {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/api/3/issuetypescheme/{issueTypeSchemeId}";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("issueTypeSchemeId", self.issue_type_scheme_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
