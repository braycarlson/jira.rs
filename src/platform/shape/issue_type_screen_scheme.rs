#[derive(Clone, Copy, Debug)]
pub struct IssueTypeScreenScheme;

impl IssueTypeScreenScheme {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const PROJECTS: crate::Field<Self, crate::Any> = crate::Field::new("projects");
}
