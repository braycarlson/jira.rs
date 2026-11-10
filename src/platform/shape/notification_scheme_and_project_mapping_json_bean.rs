#[derive(Clone, Copy, Debug)]
pub struct NotificationSchemeAndProjectMappingJSONBean;

impl NotificationSchemeAndProjectMappingJSONBean {
    pub const NOTIFICATION_SCHEME_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("notificationSchemeId");

    pub const PROJECT_ID: crate::Field<Self, crate::Text> = crate::Field::new("projectId");
}
