#[derive(Clone, Copy, Debug)]
pub struct BulkAgentSessionItemError;

impl BulkAgentSessionItemError {
    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("message");
    pub const STATUS_CODE: crate::Field<Self, crate::Integer> = crate::Field::new("statusCode");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
