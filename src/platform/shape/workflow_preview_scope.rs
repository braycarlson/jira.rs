#[derive(Clone, Copy, Debug)]
pub struct WorkflowPreviewScope;

impl WorkflowPreviewScope {
    pub const PROJECT: crate::Field<Self, crate::Object<super::WorkflowProjectIDScope>> =
        crate::Field::new("project");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
