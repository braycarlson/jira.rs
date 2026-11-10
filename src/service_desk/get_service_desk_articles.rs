#[derive(Clone, Copy, Debug)]
pub struct GetServiceDeskArticles<'call> {
    pub query: &'call str,
    pub service_desk_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub cursor: Option<&'options str>,
    pub highlight: Option<bool>,
    pub limit: Option<i64>,
    pub previous: Option<bool>,
    pub start: Option<i64>,
}

impl crate::Operation for GetServiceDeskArticles<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;

    const PATH: &'static str =
        "/rest/servicedeskapi/servicedesk/{serviceDeskId}/knowledgebase/article";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PagedDTOArticleDTO>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("cursor", options.cursor);
        request.query_boolean("highlight", options.highlight);
        request.query_number("limit", options.limit);
        request.query_boolean("prev", options.previous);
        request.query_text("query", Some(self.query));
        request.path_text("serviceDeskId", self.service_desk_id)?;
        request.query_number("start", options.start);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetServiceDeskArticles<'_> {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "cursor",
        items: Some("/values"),
        next: crate::Next::Link("/_links/next"),
    };

    type Item = crate::Object<super::shape::ArticleDTO>;
}
