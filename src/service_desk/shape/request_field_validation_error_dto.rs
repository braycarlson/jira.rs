#[derive(Clone, Copy, Debug)]
pub struct RequestFieldValidationErrorDTO;

impl RequestFieldValidationErrorDTO {
    pub const FIELD: crate::Field<Self, crate::Text> = crate::Field::new("field");
    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("message");
}
