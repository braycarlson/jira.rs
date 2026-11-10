#[derive(Clone, Copy, Debug)]
pub struct PropertyKeys;

impl PropertyKeys {
    pub const ENTITY_PROPERTY_KEY_BEANS: crate::Field<Self, crate::Any> =
        crate::Field::new("entityPropertyKeyBeans");

    pub const KEYS: crate::Field<Self, crate::List<crate::Object<super::PropertyKey>>> =
        crate::Field::new("keys");
}
