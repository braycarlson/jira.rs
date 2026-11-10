#[derive(Clone, Copy, Debug)]
pub struct JiraColorInput;

impl JiraColorInput {
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
