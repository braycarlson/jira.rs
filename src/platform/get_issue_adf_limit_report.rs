#[derive(Clone, Copy, Debug)]
pub struct GetIssueADFLimitReport;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub field_type: Option<&'options [&'options str]>,
    pub is_returning_keys: Option<bool>,
}

impl crate::Operation for GetIssueADFLimitReport {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/issue/limit/adf/report";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::IssueLimitReportResponseBean>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_texts("fieldType", options.field_type);
        request.query_boolean("isReturningKeys", options.is_returning_keys);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
