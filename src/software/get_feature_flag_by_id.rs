#[derive(Clone, Copy, Debug)]
pub struct GetFeatureFlagByID<'call> {
    pub feature_flag_id: &'call str,
}

impl crate::Operation for GetFeatureFlagByID<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/featureflags/0.1/flag/{featureFlagId}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::GetFeatureFlagByIDReply>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("featureFlagId", self.feature_flag_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
