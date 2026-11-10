#[derive(Clone, Copy, Debug)]
pub struct MandatoryFieldValueForADF;

impl MandatoryFieldValueForADF {
    pub const RETAIN: crate::Field<Self, crate::Boolean> = crate::Field::new("retain");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const VALUE: crate::Field<Self, crate::Any> = crate::Field::new("value");
}
