#[derive(Clone, Copy, Debug)]
pub struct GetAllFieldConfigurations;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub id: Option<&'options [i64]>,
    pub is_default: Option<bool>,
    pub query: Option<&'options str>,
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
}

impl crate::Operation for GetAllFieldConfigurations {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/fieldconfiguration";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PageBeanFieldConfigurationDetails>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_numbers("id", options.id);
        request.query_boolean("isDefault", options.is_default);
        request.query_text("query", options.query);
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetAllFieldConfigurations {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::FieldConfigurationDetails>;
}
