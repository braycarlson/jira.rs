#[derive(Clone, Copy, Debug)]
pub struct ScopePayload;

impl ScopePayload {
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
