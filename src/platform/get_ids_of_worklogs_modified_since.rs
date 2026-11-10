#[derive(Clone, Copy, Debug)]
pub struct GetIDsOfWorklogsModifiedSince;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options str>,
    pub since: Option<i64>,
}

impl crate::Operation for GetIDsOfWorklogsModifiedSince {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/worklog/updated";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::ChangedWorklogs>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("expand", options.expand);
        request.query_number("since", options.since);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetIDsOfWorklogsModifiedSince {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "since",
        items: Some("/values"),
        next: crate::Next::Link("/nextPage"),
    };

    type Item = crate::Object<super::shape::ChangedWorklog>;
}
