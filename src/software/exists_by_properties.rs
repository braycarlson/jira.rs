#[derive(Clone, Copy, Debug)]
pub struct ExistsByProperties;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub update_sequence_id: Option<i64>,
}

impl crate::Operation for ExistsByProperties {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/devinfo/0.10/existsByProperties";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::ExistsByPropertiesReply>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_number("_updateSequenceId", options.update_sequence_id);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
