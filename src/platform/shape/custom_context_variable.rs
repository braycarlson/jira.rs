#[derive(Clone, Copy, Debug)]
pub struct CustomContextVariable;

impl CustomContextVariable {
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
