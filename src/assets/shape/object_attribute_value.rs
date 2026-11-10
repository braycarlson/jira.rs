#[derive(Clone, Copy, Debug)]
pub struct ObjectAttributeValue;

impl ObjectAttributeValue {
    pub const ADDITIONAL_VALUE: crate::Field<Self, crate::Text> =
        crate::Field::new("additionalValue");

    pub const DISPLAY_VALUE: crate::Field<Self, crate::Text> = crate::Field::new("displayValue");
    pub const GROUP: crate::Field<Self, crate::Object<super::Group>> = crate::Field::new("group");

    pub const REFERENCED_OBJECT: crate::Field<Self, crate::Any> =
        crate::Field::new("referencedObject");

    pub const REFERENCED_TYPE: crate::Field<Self, crate::Boolean> =
        crate::Field::new("referencedType");

    pub const SEARCH_VALUE: crate::Field<Self, crate::Text> = crate::Field::new("searchValue");

    pub const STATUS: crate::Field<Self, crate::Object<super::Status>> =
        crate::Field::new("status");

    pub const USER: crate::Field<Self, crate::Object<super::User>> = crate::Field::new("user");
    pub const VALUE: crate::Field<Self, crate::Any> = crate::Field::new("value");
}
