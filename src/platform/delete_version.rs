#[derive(Clone, Copy, Debug)]
pub struct DeleteVersion<'call> {
    pub id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub move_affected_issues_to: Option<&'options str>,
    pub move_fix_issues_to: Option<&'options str>,
}

impl crate::Operation for DeleteVersion<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/api/3/version/{id}";

    type Options<'options> = Options<'options>;
    type Reply = crate::Nothing;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("id", self.id)?;
        request.query_text("moveAffectedIssuesTo", options.move_affected_issues_to);
        request.query_text("moveFixIssuesTo", options.move_fix_issues_to);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
