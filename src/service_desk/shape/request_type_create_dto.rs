#[derive(Clone, Copy, Debug)]
pub struct RequestTypeCreateDTO;

impl RequestTypeCreateDTO {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const HELP_TEXT: crate::Field<Self, crate::Text> = crate::Field::new("helpText");
    pub const ISSUE_TYPE_ID: crate::Field<Self, crate::Text> = crate::Field::new("issueTypeId");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
