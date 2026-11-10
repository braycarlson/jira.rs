#[derive(Clone, Copy, Debug)]
pub struct JiraWorkflowPreviewStatus;

impl JiraWorkflowPreviewStatus {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const RAW_NAME: crate::Field<Self, crate::Text> = crate::Field::new("rawName");

    pub const SCOPE: crate::Field<Self, crate::Object<super::WorkflowPreviewScope>> =
        crate::Field::new("scope");

    pub const STATUS_CATEGORY: crate::Field<Self, crate::Text> =
        crate::Field::new("statusCategory");

    pub const STATUS_REFERENCE: crate::Field<Self, crate::Text> =
        crate::Field::new("statusReference");
}
