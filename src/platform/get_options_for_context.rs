#[derive(Clone, Copy, Debug)]
pub struct GetOptionsForContext<'call> {
    pub context_id: i64,
    pub field_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub only_options: Option<bool>,
    pub option_id: Option<i64>,
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
}

impl crate::Operation for GetOptionsForContext<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/field/{fieldId}/context/{contextId}/option";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::PageBeanCustomFieldContextOption>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("contextId", self.context_id)?;
        request.path_text("fieldId", self.field_id)?;
        request.query_boolean("onlyOptions", options.only_options);
        request.query_number("optionId", options.option_id);
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetOptionsForContext<'_> {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::CustomFieldContextOption>;
}
