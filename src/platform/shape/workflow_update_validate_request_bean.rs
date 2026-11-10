#[derive(Clone, Copy, Debug)]
pub struct WorkflowUpdateValidateRequestBean;

impl WorkflowUpdateValidateRequestBean {
    pub const PAYLOAD: crate::Field<Self, crate::Object<super::WorkflowUpdateRequest>> =
        crate::Field::new("payload");

    pub const VALIDATION_OPTIONS: crate::Field<
        Self,
        crate::Object<super::ValidationOptionsForUpdate>,
    > = crate::Field::new("validationOptions");
}
