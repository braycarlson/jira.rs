#[derive(Clone, Copy, Debug)]
pub struct WorkflowSchemeIDName;

impl WorkflowSchemeIDName {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
