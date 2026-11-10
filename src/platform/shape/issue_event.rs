#[derive(Clone, Copy, Debug)]
pub struct IssueEvent;

impl IssueEvent {
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
