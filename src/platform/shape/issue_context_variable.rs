#[derive(Clone, Copy, Debug)]
pub struct IssueContextVariable;

impl IssueContextVariable {
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
