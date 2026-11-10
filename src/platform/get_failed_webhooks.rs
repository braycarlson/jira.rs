#[derive(Clone, Copy, Debug)]
pub struct GetFailedWebhooks;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub after: Option<i64>,
    pub results_max: Option<i64>,
}

impl crate::Operation for GetFailedWebhooks {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/webhook/failed";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::FailedWebhooks>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_number("after", options.after);
        request.query_number("maxResults", options.results_max);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetFailedWebhooks {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "after", items: Some("/values"), next: crate::Next::Link("/next") };

    type Item = crate::Object<super::shape::FailedWebhook>;
}
