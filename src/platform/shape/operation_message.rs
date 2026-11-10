#[derive(Clone, Copy, Debug)]
pub struct OperationMessage;

impl OperationMessage {
    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("message");
    pub const STATUS_CODE: crate::Field<Self, crate::Integer> = crate::Field::new("statusCode");
}
