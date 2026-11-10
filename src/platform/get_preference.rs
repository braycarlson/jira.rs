#[derive(Clone, Copy, Debug)]
pub struct GetPreference<'call> {
    pub key: &'call str,
}

impl crate::Operation for GetPreference<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/mypreferences";

    type Options<'options> = ();
    type Reply = crate::Text;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("key", Some(self.key));

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
