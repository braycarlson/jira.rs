#[derive(Clone, Copy, Debug)]
pub struct WorkflowPreviewStatus;

impl WorkflowPreviewStatus {
    pub const APPROVAL_CONFIGURATION: crate::Field<
        Self,
        crate::Object<super::ApprovalConfigurationPreview>,
    > = crate::Field::new("approvalConfiguration");

    pub const DEPRECATED: crate::Field<Self, crate::Boolean> = crate::Field::new("deprecated");

    pub const LAYOUT: crate::Field<Self, crate::Object<super::WorkflowPreviewLayout>> =
        crate::Field::new("layout");

    pub const STATUS_REFERENCE: crate::Field<Self, crate::Text> =
        crate::Field::new("statusReference");
}
