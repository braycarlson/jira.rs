#[derive(Clone, Copy, Debug)]
pub struct GetFieldAutoCompleteForQueryString;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub field_name: Option<&'options str>,
    pub field_value: Option<&'options str>,
    pub predicate_name: Option<&'options str>,
    pub predicate_value: Option<&'options str>,
}

impl crate::Operation for GetFieldAutoCompleteForQueryString {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/jql/autocompletedata/suggestions";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::AutoCompleteSuggestions>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("fieldName", options.field_name);
        request.query_text("fieldValue", options.field_value);
        request.query_text("predicateName", options.predicate_name);
        request.query_text("predicateValue", options.predicate_value);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
