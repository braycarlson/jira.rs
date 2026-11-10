#[derive(Clone, Copy, Debug)]
pub struct IssueError;

impl IssueError {
    pub const ERROR_MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("errorMessage");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
