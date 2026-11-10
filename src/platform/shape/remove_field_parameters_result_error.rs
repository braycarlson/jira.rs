#[derive(Clone, Copy, Debug)]
pub struct RemoveFieldParametersResultError;

impl RemoveFieldParametersResultError {
    pub const CODE: crate::Field<Self, crate::Text> = crate::Field::new("code");
    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("message");
}
