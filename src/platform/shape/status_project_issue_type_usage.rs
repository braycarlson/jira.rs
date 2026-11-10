#[derive(Clone, Copy, Debug)]
pub struct StatusProjectIssueTypeUsage;

impl StatusProjectIssueTypeUsage {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
