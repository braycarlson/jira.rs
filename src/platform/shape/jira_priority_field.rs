#[derive(Clone, Copy, Debug)]
pub struct JiraPriorityField;

impl JiraPriorityField {
    pub const PRIORITY_ID: crate::Field<Self, crate::Text> = crate::Field::new("priorityId");
}
