#[derive(Clone, Copy, Debug)]
pub struct JiraComponentField;

impl JiraComponentField {
    pub const COMPONENT_ID: crate::Field<Self, crate::Integer> = crate::Field::new("componentId");
}
