#[derive(Clone, Copy, Debug)]
pub struct GetNotificationScheme {
    pub id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options str>,
}

impl crate::Operation for GetNotificationScheme {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/notificationscheme/{id}";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::NotificationScheme>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("expand", options.expand);
        request.path_number("id", self.id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
