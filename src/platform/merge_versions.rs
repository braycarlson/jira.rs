#[derive(Clone, Copy, Debug)]
pub struct MergeVersions<'call> {
    pub id: &'call str,
    pub move_issues_to: &'call str,
}

impl crate::Operation for MergeVersions<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
    const PATH: &'static str = "/rest/api/3/version/{id}/mergeto/{moveIssuesTo}";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("id", self.id)?;
        request.path_text("moveIssuesTo", self.move_issues_to)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
