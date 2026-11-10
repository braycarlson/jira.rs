#[derive(Clone, Copy, Debug)]
pub struct GetBulkEditableFields<'call> {
    pub issue_ids_or_keys: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub ending_before: Option<&'options str>,
    pub search_text: Option<&'options str>,
    pub starting_after: Option<&'options str>,
}

impl crate::Operation for GetBulkEditableFields<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/bulk/issues/fields";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::BulkEditGetFields>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("endingBefore", options.ending_before);
        request.query_text("issueIdsOrKeys", Some(self.issue_ids_or_keys));
        request.query_text("searchText", options.search_text);
        request.query_text("startingAfter", options.starting_after);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
