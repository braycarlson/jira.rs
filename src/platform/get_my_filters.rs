#[derive(Clone, Copy, Debug)]
pub struct GetMyFilters;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options str>,
    pub include_favourites: Option<bool>,
}

impl crate::Operation for GetMyFilters {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/filter/my";

    type Options<'options> = Options<'options>;
    type Reply = crate::List<crate::Object<super::shape::Filter>>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("expand", options.expand);
        request.query_boolean("includeFavourites", options.include_favourites);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
