#[derive(Clone, Copy, Debug)]
pub struct GetNotificationSchemeToProjectMappings;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub notification_scheme_id: Option<&'options [&'options str]>,
    pub project_id: Option<&'options [&'options str]>,
    pub results_max: Option<&'options str>,
    pub start_at: Option<&'options str>,
}

impl crate::Operation for GetNotificationSchemeToProjectMappings {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/notificationscheme/project";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PageBeanNotificationSchemeAndProjectMappingJSONBean>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_texts("notificationSchemeId", options.notification_scheme_id);
        request.query_texts("projectId", options.project_id);
        request.query_text("maxResults", options.results_max);
        request.query_text("startAt", options.start_at);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetNotificationSchemeToProjectMappings {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::NotificationSchemeAndProjectMappingJSONBean>;
}
