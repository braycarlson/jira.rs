#[derive(Clone, Copy, Debug)]
pub struct WorkflowValidationError;

impl WorkflowValidationError {
    pub const ADDITIONAL_DETAILS: crate::Field<Self, crate::Text> =
        crate::Field::new("additionalDetails");

    pub const CODE: crate::Field<Self, crate::Text> = crate::Field::new("code");

    pub const ELEMENT_REFERENCE: crate::Field<
        Self,
        crate::Object<super::WorkflowElementReference>,
    > = crate::Field::new("elementReference");

    pub const LEVEL: crate::Field<Self, crate::Text> = crate::Field::new("level");
    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("message");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
