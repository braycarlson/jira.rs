#[derive(Clone, Copy, Debug)]
pub struct GetUserEmailBulk<'call> {
    pub account_id: &'call [&'call str],
}

impl crate::Operation for GetUserEmailBulk<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/user/email/bulk";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::UnrestrictedUserEmail>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_texts("accountId", Some(self.account_id));

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
