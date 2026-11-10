#[derive(Clone, Copy, Debug)]
pub struct DeleteFeatureFlagsByProperty<'call> {
    pub properties: &'call [(&'call str, &'call str)],
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub update_sequence_id: Option<i64>,
}

impl crate::Operation for DeleteFeatureFlagsByProperty<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/featureflags/0.1/bulkByProperties";

    type Options<'options> = Options;
    type Reply = crate::Nothing;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_pairs(self.properties)?;
        request.query_number("_updateSequenceId", options.update_sequence_id);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
