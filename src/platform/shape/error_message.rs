#[derive(Clone, Copy, Debug)]
pub struct ErrorMessage;

impl ErrorMessage {
    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("message");
}
