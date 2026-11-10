#[derive(Clone, Copy, Debug)]
pub struct FindUserKeysByQuery<'call> {
    pub query: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub result_max: Option<i64>,
    pub start_at: Option<i64>,
}

impl crate::Operation for FindUserKeysByQuery<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/user/search/query/key";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::PageBeanUserKey>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("query", Some(self.query));
        request.query_number("maxResult", options.result_max);
        request.query_number("startAt", options.start_at);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
