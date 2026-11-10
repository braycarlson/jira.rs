#[derive(Clone, Copy, Debug)]
pub struct JiraLabelsInput;

impl JiraLabelsInput {
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
