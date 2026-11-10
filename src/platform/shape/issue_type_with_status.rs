#[derive(Clone, Copy, Debug)]
pub struct IssueTypeWithStatus;

impl IssueTypeWithStatus {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");

    pub const STATUSES: crate::Field<Self, crate::List<crate::Object<super::StatusDetails>>> =
        crate::Field::new("statuses");

    pub const SUBTASK: crate::Field<Self, crate::Boolean> = crate::Field::new("subtask");
}
