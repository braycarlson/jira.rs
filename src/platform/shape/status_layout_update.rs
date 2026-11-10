#[derive(Clone, Copy, Debug)]
pub struct StatusLayoutUpdate;

impl StatusLayoutUpdate {
    pub const APPROVAL_CONFIGURATION: crate::Field<
        Self,
        crate::Object<super::ApprovalConfiguration>,
    > = crate::Field::new("approvalConfiguration");

    pub const LAYOUT: crate::Field<Self, crate::Object<super::WorkflowLayout>> =
        crate::Field::new("layout");

    pub const PROPERTIES: crate::Field<Self, crate::Map<crate::Text>> =
        crate::Field::new("properties");

    pub const STATUS_REFERENCE: crate::Field<Self, crate::Text> =
        crate::Field::new("statusReference");
}
