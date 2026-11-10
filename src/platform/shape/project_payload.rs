#[derive(Clone, Copy, Debug)]
pub struct ProjectPayload;

impl ProjectPayload {
    pub const FIELD_LAYOUT_SCHEME_ID: crate::Field<
        Self,
        crate::Object<super::ProjectCreateResourceIdentifier>,
    > = crate::Field::new("fieldLayoutSchemeId");

    pub const ISSUE_SECURITY_SCHEME_ID: crate::Field<
        Self,
        crate::Object<super::ProjectCreateResourceIdentifier>,
    > = crate::Field::new("issueSecuritySchemeId");

    pub const ISSUE_TYPE_SCHEME_ID: crate::Field<
        Self,
        crate::Object<super::ProjectCreateResourceIdentifier>,
    > = crate::Field::new("issueTypeSchemeId");

    pub const ISSUE_TYPE_SCREEN_SCHEME_ID: crate::Field<
        Self,
        crate::Object<super::ProjectCreateResourceIdentifier>,
    > = crate::Field::new("issueTypeScreenSchemeId");

    pub const NOTIFICATION_SCHEME_ID: crate::Field<
        Self,
        crate::Object<super::ProjectCreateResourceIdentifier>,
    > = crate::Field::new("notificationSchemeId");

    pub const PCRI: crate::Field<Self, crate::Object<super::ProjectCreateResourceIdentifier>> =
        crate::Field::new("pcri");

    pub const PERMISSION_SCHEME_ID: crate::Field<
        Self,
        crate::Object<super::ProjectCreateResourceIdentifier>,
    > = crate::Field::new("permissionSchemeId");

    pub const PROJECT_TYPE_KEY: crate::Field<Self, crate::Text> =
        crate::Field::new("projectTypeKey");

    pub const WORKFLOW_SCHEME_ID: crate::Field<
        Self,
        crate::Object<super::ProjectCreateResourceIdentifier>,
    > = crate::Field::new("workflowSchemeId");
}
