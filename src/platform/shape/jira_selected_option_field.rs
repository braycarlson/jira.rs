#[derive(Clone, Copy, Debug)]
pub struct JiraSelectedOptionField;

impl JiraSelectedOptionField {
    pub const OPTION_ID: crate::Field<Self, crate::Integer> = crate::Field::new("optionId");
}
