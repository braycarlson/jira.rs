#[derive(Clone, Copy, Debug)]
pub struct CustomTemplateRequestDTO;

impl CustomTemplateRequestDTO {
    pub const BOARDS: crate::Field<Self, crate::Object<super::BoardsPayload>> =
        crate::Field::new("boards");

    pub const BOARD_FEATURES: crate::Field<Self, crate::Object<super::BoardFeaturesPayload>> =
        crate::Field::new("boardFeatures");

    pub const FIELD: crate::Field<Self, crate::Object<super::FieldCapabilityPayload>> =
        crate::Field::new("field");

    pub const ISSUE_TYPE: crate::Field<Self, crate::Object<super::IssueTypeProjectCreatePayload>> =
        crate::Field::new("issueType");

    pub const NOTIFICATION: crate::Field<Self, crate::Object<super::NotificationSchemePayload>> =
        crate::Field::new("notification");

    pub const PERMISSION_SCHEME: crate::Field<Self, crate::Object<super::PermissionPayloadDTO>> =
        crate::Field::new("permissionScheme");

    pub const PROJECT: crate::Field<Self, crate::Object<super::ProjectPayload>> =
        crate::Field::new("project");

    pub const ROLE: crate::Field<Self, crate::Object<super::RolesCapabilityPayload>> =
        crate::Field::new("role");

    pub const SCOPE: crate::Field<Self, crate::Object<super::ScopePayload>> =
        crate::Field::new("scope");

    pub const SECURITY: crate::Field<Self, crate::Object<super::SecuritySchemePayload>> =
        crate::Field::new("security");

    pub const WORKFLOW: crate::Field<Self, crate::Object<super::WorkflowCapabilityPayload>> =
        crate::Field::new("workflow");
}
