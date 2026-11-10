#[derive(Clone, Copy, Debug)]
pub struct VersionIssuesStatus;

impl VersionIssuesStatus {
    pub const DONE: crate::Field<Self, crate::Integer> = crate::Field::new("done");
    pub const IN_PROGRESS: crate::Field<Self, crate::Integer> = crate::Field::new("inProgress");
    pub const TO_DO: crate::Field<Self, crate::Integer> = crate::Field::new("toDo");
    pub const UNMAPPED: crate::Field<Self, crate::Integer> = crate::Field::new("unmapped");
}
