#[derive(Clone, Copy, Debug)]
pub struct DeleteRemoteLinkByID<'call> {
    pub remote_link_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub update_sequence_number: Option<i64>,
}

impl crate::Operation for DeleteRemoteLinkByID<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/remotelinks/1.0/remotelink/{remoteLinkId}";

    type Options<'options> = Options;
    type Reply = crate::Nothing;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("remoteLinkId", self.remote_link_id)?;
        request.query_number("_updateSequenceNumber", options.update_sequence_number);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
