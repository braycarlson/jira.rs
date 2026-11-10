#[derive(Clone, Copy, Debug)]
pub struct CreateDashboard<'call> {
    pub body: &'call serde_json::Value,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub extend_admin_permissions: Option<bool>,
}

impl crate::Operation for CreateDashboard<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/api/3/dashboard";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::Dashboard>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_boolean("extendAdminPermissions", options.extend_admin_permissions);
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
