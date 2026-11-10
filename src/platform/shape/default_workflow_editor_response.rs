#[derive(Clone, Copy, Debug)]
pub struct DefaultWorkflowEditorResponse;

impl DefaultWorkflowEditorResponse {
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
