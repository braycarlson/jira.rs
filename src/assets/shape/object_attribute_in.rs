#[derive(Clone, Copy, Debug)]
pub struct ObjectAttributeIn;

impl ObjectAttributeIn {
    pub const OBJECT_ATTRIBUTE_VALUES: crate::Field<
        Self,
        crate::List<crate::Object<super::ObjectAttributeValueIn>>,
    > = crate::Field::new("objectAttributeValues");

    pub const OBJECT_TYPE_ATTRIBUTE_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("objectTypeAttributeId");
}
