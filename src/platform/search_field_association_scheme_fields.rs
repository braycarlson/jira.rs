#[derive(Clone, Copy, Debug)]
pub struct SearchFieldAssociationSchemeFields {
    pub id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub field_id: Option<&'options [&'options str]>,
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
}

impl crate::Operation for SearchFieldAssociationSchemeFields {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/config/fieldschemes/{id}/fields";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PageBean2FieldAssociationSchemeFieldSearchResult>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_texts("fieldId", options.field_id);
        request.path_number("id", self.id)?;
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for SearchFieldAssociationSchemeFields {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::FieldAssociationSchemeFieldSearchResult>;
}
