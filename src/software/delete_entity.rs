#[derive(Clone, Copy, Debug)]
pub struct DeleteEntity<'call> {
    pub entity_id: &'call str,
    pub entity_type: &'call str,
    pub repository_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub update_sequence_id: Option<i64>,
}

impl crate::Operation for DeleteEntity<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;

    const PATH: &'static str =
        "/rest/devinfo/0.10/repository/{repositoryId}/{entityType}/{entityId}";

    type Options<'options> = Options;
    type Reply = crate::Nothing;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("entityId", self.entity_id)?;
        request.path_text("entityType", self.entity_type)?;
        request.path_text("repositoryId", self.repository_id)?;
        request.query_number("_updateSequenceId", options.update_sequence_id);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
