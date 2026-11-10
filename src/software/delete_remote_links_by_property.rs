#[derive(Clone, Copy, Debug)]
pub struct DeleteRemoteLinksByProperty<'call> {
    pub properties: &'call [(&'call str, &'call str)],
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub update_sequence_number: Option<i64>,
}

impl crate::Operation for DeleteRemoteLinksByProperty<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/remotelinks/1.0/bulkByProperties";

    type Options<'options> = Options;
    type Reply = crate::Nothing;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_pairs(self.properties)?;
        request.query_number("_updateSequenceNumber", options.update_sequence_number);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
