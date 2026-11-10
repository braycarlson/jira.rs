#[derive(Clone, Copy, Debug)]
pub struct WorkflowValidationErrorList;

impl WorkflowValidationErrorList {
    pub const ERRORS: crate::Field<
        Self,
        crate::List<crate::Object<super::WorkflowValidationError>>,
    > = crate::Field::new("errors");
}
