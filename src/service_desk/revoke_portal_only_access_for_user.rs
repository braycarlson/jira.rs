#[derive(Clone, Copy, Debug)]
pub struct RevokePortalOnlyAccessForUser<'call> {
    pub account_id: &'call str,
}

impl crate::Operation for RevokePortalOnlyAccessForUser<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;

    const PATH: &'static str =
        "/rest/servicedeskapi/customer/user/{accountId}/revoke-portal-only-access";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("accountId", self.account_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
