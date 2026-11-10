#[derive(Clone, Copy, Debug)]
pub struct BulkGetGroups;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub access_type: Option<&'options str>,
    pub application_key: Option<&'options str>,
    pub group_id: Option<&'options [&'options str]>,
    pub group_name: Option<&'options [&'options str]>,
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
}

impl crate::Operation for BulkGetGroups {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/group/bulk";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PageBeanGroupDetails>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("accessType", options.access_type);
        request.query_text("applicationKey", options.application_key);
        request.query_texts("groupId", options.group_id);
        request.query_texts("groupName", options.group_name);
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for BulkGetGroups {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::GroupDetails>;
}
