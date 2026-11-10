#[derive(Clone, Copy, Debug)]
pub struct ObjectAttributeValueIn;

impl ObjectAttributeValueIn {
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
