#[derive(Clone, Copy, Debug)]
pub struct WorkflowUpdateRequest;

impl WorkflowUpdateRequest {
    pub const STATUSES: crate::Field<
        Self,
        crate::List<crate::Object<super::WorkflowStatusUpdate>>,
    > = crate::Field::new("statuses");

    pub const WORKFLOWS: crate::Field<Self, crate::List<crate::Object<super::WorkflowUpdate>>> =
        crate::Field::new("workflows");
}
