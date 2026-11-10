#[derive(Clone, Copy, Debug)]
pub struct GetNotificationSchemeForProject<'call> {
    pub project_key_or_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options str>,
}

impl crate::Operation for GetNotificationSchemeForProject<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/project/{projectKeyOrId}/notificationscheme";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::NotificationScheme>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("expand", options.expand);
        request.path_text("projectKeyOrId", self.project_key_or_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
