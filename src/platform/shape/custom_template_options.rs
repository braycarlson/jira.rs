#[derive(Clone, Copy, Debug)]
pub struct CustomTemplateOptions;

impl CustomTemplateOptions {
    pub const ENABLE_SCREEN_DELEGATED_ADMIN_SUPPORT: crate::Field<Self, crate::Boolean> =
        crate::Field::new("enableScreenDelegatedAdminSupport");

    pub const ENABLE_WORKFLOW_DELEGATED_ADMIN_SUPPORT: crate::Field<Self, crate::Boolean> =
        crate::Field::new("enableWorkflowDelegatedAdminSupport");
}
