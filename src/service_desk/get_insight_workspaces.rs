#[derive(Clone, Copy, Debug)]
pub struct GetInsightWorkspaces;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub limit: Option<i64>,
    pub start: Option<i64>,
}

impl crate::Operation for GetInsightWorkspaces {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/servicedeskapi/insight/workspace";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::PagedDTOInsightWorkspaceDTO>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_number("limit", options.limit);
        request.query_number("start", options.start);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetInsightWorkspaces {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "start", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::InsightWorkspaceDTO>;
}
