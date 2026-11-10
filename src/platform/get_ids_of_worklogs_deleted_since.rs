#[derive(Clone, Copy, Debug)]
pub struct GetIDsOfWorklogsDeletedSince;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub since: Option<i64>,
}

impl crate::Operation for GetIDsOfWorklogsDeletedSince {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/worklog/deleted";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::ChangedWorklogs>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_number("since", options.since);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetIDsOfWorklogsDeletedSince {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "since",
        items: Some("/values"),
        next: crate::Next::Link("/nextPage"),
    };

    type Item = crate::Object<super::shape::ChangedWorklog>;
}
