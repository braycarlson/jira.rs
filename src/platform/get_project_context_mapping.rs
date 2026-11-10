#[derive(Clone, Copy, Debug)]
pub struct GetProjectContextMapping<'call> {
    pub field_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub context_id: Option<&'options [i64]>,
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
}

impl crate::Operation for GetProjectContextMapping<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/field/{fieldId}/context/projectmapping";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PageBeanCustomFieldContextProjectMapping>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_numbers("contextId", options.context_id);
        request.path_text("fieldId", self.field_id)?;
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetProjectContextMapping<'_> {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::CustomFieldContextProjectMapping>;
}
