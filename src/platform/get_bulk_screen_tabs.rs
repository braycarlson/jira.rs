#[derive(Clone, Copy, Debug)]
pub struct GetBulkScreenTabs;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub result_max: Option<i64>,
    pub screen_id: Option<&'options [i64]>,
    pub start_at: Option<i64>,
    pub tab_id: Option<&'options [i64]>,
}

impl crate::Operation for GetBulkScreenTabs {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/screens/tabs";

    type Options<'options> = Options<'options>;
    type Reply = crate::Any;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_number("maxResult", options.result_max);
        request.query_numbers("screenId", options.screen_id);
        request.query_number("startAt", options.start_at);
        request.query_numbers("tabId", options.tab_id);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
