#[derive(Clone, Copy, Debug)]
pub struct WorkflowCreateValidateRequest;

impl WorkflowCreateValidateRequest {
    pub const PAYLOAD: crate::Field<Self, crate::Object<super::WorkflowCreateRequest>> =
        crate::Field::new("payload");

    pub const VALIDATION_OPTIONS: crate::Field<
        Self,
        crate::Object<super::ValidationOptionsForCreate>,
    > = crate::Field::new("validationOptions");
}
