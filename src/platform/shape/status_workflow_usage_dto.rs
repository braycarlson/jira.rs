#[derive(Clone, Copy, Debug)]
pub struct StatusWorkflowUsageDTO;

impl StatusWorkflowUsageDTO {
    pub const STATUS_ID: crate::Field<Self, crate::Text> = crate::Field::new("statusId");

    pub const WORKFLOWS: crate::Field<Self, crate::Object<super::StatusWorkflowUsagePage>> =
        crate::Field::new("workflows");
}
