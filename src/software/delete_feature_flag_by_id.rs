#[derive(Clone, Copy, Debug)]
pub struct DeleteFeatureFlagByID<'call> {
    pub feature_flag_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub update_sequence_id: Option<i64>,
}

impl crate::Operation for DeleteFeatureFlagByID<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/featureflags/0.1/flag/{featureFlagId}";

    type Options<'options> = Options;
    type Reply = crate::Nothing;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("featureFlagId", self.feature_flag_id)?;
        request.query_number("_updateSequenceId", options.update_sequence_id);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
