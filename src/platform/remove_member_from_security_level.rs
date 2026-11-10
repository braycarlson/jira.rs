#[derive(Clone, Copy, Debug)]
pub struct RemoveMemberFromSecurityLevel<'call> {
    pub level_id: &'call str,
    pub member_id: &'call str,
    pub scheme_id: &'call str,
}

impl crate::Operation for RemoveMemberFromSecurityLevel<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;

    const PATH: &'static str =
        "/rest/api/3/issuesecurityschemes/{schemeId}/level/{levelId}/member/{memberId}";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("levelId", self.level_id)?;
        request.path_text("memberId", self.member_id)?;
        request.path_text("schemeId", self.scheme_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
