#[derive(Clone, Copy, Debug)]
pub struct WorkflowReferenceStatus;

impl WorkflowReferenceStatus {
    pub const APPROVAL_CONFIGURATION: crate::Field<
        Self,
        crate::Object<super::ApprovalConfiguration>,
    > = crate::Field::new("approvalConfiguration");

    pub const DEPRECATED: crate::Field<Self, crate::Boolean> = crate::Field::new("deprecated");

    pub const LAYOUT: crate::Field<Self, crate::Object<super::WorkflowStatusLayout>> =
        crate::Field::new("layout");

    pub const PROPERTIES: crate::Field<Self, crate::Map<crate::Text>> =
        crate::Field::new("properties");

    pub const STATUS_REFERENCE: crate::Field<Self, crate::Text> =
        crate::Field::new("statusReference");
}
