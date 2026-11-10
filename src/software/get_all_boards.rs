#[derive(Clone, Copy, Debug)]
pub struct GetAllBoards;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub account_id_location: Option<&'options str>,
    pub expand: Option<&'options str>,
    pub filter_id: Option<i64>,
    pub include_private: Option<bool>,
    pub kind: Option<&'options str>,
    pub name: Option<&'options str>,
    pub negate_location_filtering: Option<bool>,
    pub order_by: Option<&'options str>,
    pub project_key_or_id: Option<&'options str>,
    pub project_location: Option<&'options str>,
    pub project_type_location: Option<&'options [&'options str]>,
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
}

impl crate::Operation for GetAllBoards {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/agile/1.0/board";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::GetAllBoardsReply>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("accountIdLocation", options.account_id_location);
        request.query_text("expand", options.expand);
        request.query_number("filterId", options.filter_id);
        request.query_boolean("includePrivate", options.include_private);
        request.query_text("type", options.kind);
        request.query_text("name", options.name);
        request.query_boolean("negateLocationFiltering", options.negate_location_filtering);
        request.query_text("orderBy", options.order_by);
        request.query_text("projectKeyOrId", options.project_key_or_id);
        request.query_text("projectLocation", options.project_location);
        request.query_texts("projectTypeLocation", options.project_type_location);
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetAllBoards {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::GetAllBoardsItem>;
}
