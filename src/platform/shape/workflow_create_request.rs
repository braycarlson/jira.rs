#[derive(Clone, Copy, Debug)]
pub struct WorkflowCreateRequest;

impl WorkflowCreateRequest {
    pub const SCOPE: crate::Field<Self, crate::Object<super::WorkflowScope>> =
        crate::Field::new("scope");

    pub const STATUSES: crate::Field<
        Self,
        crate::List<crate::Object<super::WorkflowStatusUpdate>>,
    > = crate::Field::new("statuses");

    pub const WORKFLOWS: crate::Field<Self, crate::List<crate::Object<super::WorkflowCreate>>> =
        crate::Field::new("workflows");
}
