#[derive(Clone, Copy, Debug)]
pub struct DeleteIssueTypeScreenScheme<'call> {
    pub issue_type_screen_scheme_id: &'call str,
}

impl crate::Operation for DeleteIssueTypeScreenScheme<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/api/3/issuetypescreenscheme/{issueTypeScreenSchemeId}";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("issueTypeScreenSchemeId", self.issue_type_screen_scheme_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
