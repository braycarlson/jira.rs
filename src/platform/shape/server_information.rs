#[derive(Clone, Copy, Debug)]
pub struct ServerInformation;

impl ServerInformation {
    pub const BASE_URL: crate::Field<Self, crate::Text> = crate::Field::new("baseUrl");
    pub const BUILD_DATE: crate::Field<Self, crate::Text> = crate::Field::new("buildDate");
    pub const BUILD_NUMBER: crate::Field<Self, crate::Integer> = crate::Field::new("buildNumber");
    pub const DEFAULT_LOCALE: crate::Field<Self, crate::Any> = crate::Field::new("defaultLocale");

    pub const DEPLOYMENT_TYPE: crate::Field<Self, crate::Text> =
        crate::Field::new("deploymentType");

    pub const DISPLAY_URL: crate::Field<Self, crate::Text> = crate::Field::new("displayUrl");

    pub const DISPLAY_URL_CONFLUENCE: crate::Field<Self, crate::Text> =
        crate::Field::new("displayUrlConfluence");

    pub const DISPLAY_URL_CSM_HELP_SEEKER: crate::Field<Self, crate::Text> =
        crate::Field::new("displayUrlCSMHelpSeeker");

    pub const DISPLAY_URL_SERVICEDESK_HELP_CENTER: crate::Field<Self, crate::Text> =
        crate::Field::new("displayUrlServicedeskHelpCenter");

    pub const HEALTH_CHECKS: crate::Field<
        Self,
        crate::List<crate::Object<super::HealthCheckResult>>,
    > = crate::Field::new("healthChecks");

    pub const SCM_INFORMATION: crate::Field<Self, crate::Text> = crate::Field::new("scmInfo");
    pub const SERVER_TIME: crate::Field<Self, crate::Text> = crate::Field::new("serverTime");

    pub const SERVER_TIME_ZONE: crate::Field<Self, crate::Text> =
        crate::Field::new("serverTimeZone");

    pub const SERVER_TITLE: crate::Field<Self, crate::Text> = crate::Field::new("serverTitle");
    pub const VERSION: crate::Field<Self, crate::Text> = crate::Field::new("version");

    pub const VERSION_NUMBERS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("versionNumbers");
}
