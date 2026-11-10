#[derive(Clone, Copy, Debug)]
pub struct GetApproximateApplicationLicenseCount<'call> {
    pub application_key: &'call str,
}

impl crate::Operation for GetApproximateApplicationLicenseCount<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;

    const PATH: &'static str =
        "/rest/api/3/license/approximateLicenseCount/product/{applicationKey}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::LicenseMetric>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("applicationKey", self.application_key)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
