#[derive(Clone, Copy, Debug)]
pub struct IssueTransitionStatus;

impl IssueTransitionStatus {
    pub const STATUS_ID: crate::Field<Self, crate::Integer> = crate::Field::new("statusId");
    pub const STATUS_NAME: crate::Field<Self, crate::Text> = crate::Field::new("statusName");
}
