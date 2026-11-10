#[derive(Clone, Copy, Debug)]
pub struct StatusMapping;

impl StatusMapping {
    pub const ISSUE_TYPE_ID: crate::Field<Self, crate::Text> = crate::Field::new("issueTypeId");
    pub const STATUS_ID: crate::Field<Self, crate::Text> = crate::Field::new("statusId");
    pub const STATUS_ID_NEW: crate::Field<Self, crate::Text> = crate::Field::new("newStatusId");
}
