#[derive(Clone, Copy, Debug)]
pub struct GetAllUserDataClassificationLevels;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub order_by: Option<&'options str>,
    pub status: Option<&'options [&'options str]>,
}

impl crate::Operation for GetAllUserDataClassificationLevels {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/classification-levels";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::DataClassificationLevelsBean>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("orderBy", options.order_by);
        request.query_texts("status", options.status);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
