#[derive(Clone, Copy, Debug)]
pub struct GetNotificationSchemes;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options str>,
    pub id: Option<&'options [&'options str]>,
    pub only_default: Option<bool>,
    pub project_id: Option<&'options [&'options str]>,
    pub results_max: Option<&'options str>,
    pub start_at: Option<&'options str>,
}

impl crate::Operation for GetNotificationSchemes {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/notificationscheme";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PageBeanNotificationScheme>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("expand", options.expand);
        request.query_texts("id", options.id);
        request.query_boolean("onlyDefault", options.only_default);
        request.query_texts("projectId", options.project_id);
        request.query_text("maxResults", options.results_max);
        request.query_text("startAt", options.start_at);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetNotificationSchemes {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::NotificationScheme>;
}
