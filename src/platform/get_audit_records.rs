#[derive(Clone, Copy, Debug)]
pub struct GetAuditRecords;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub filter: Option<&'options str>,
    pub from: Option<&'options str>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub to_value: Option<&'options str>,
}

impl crate::Operation for GetAuditRecords {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/auditing/record";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::AuditRecords>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("filter", options.filter);
        request.query_text("from", options.from);
        request.query_number("limit", options.limit);
        request.query_number("offset", options.offset);
        request.query_text("to", options.to_value);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetAuditRecords {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "offset", items: Some("/records"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::AuditRecordBean>;
}
