#[derive(Clone, Copy, Debug)]
pub struct MandatoryFieldValue;

impl MandatoryFieldValue {
    pub const RETAIN: crate::Field<Self, crate::Boolean> = crate::Field::new("retain");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const VALUE: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("value");
}
