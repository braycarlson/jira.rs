#[derive(Clone, Copy, Debug)]
pub struct WorkflowScope;

impl WorkflowScope {
    pub const PROJECT: crate::Field<Self, crate::Object<super::ProjectID>> =
        crate::Field::new("project");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
