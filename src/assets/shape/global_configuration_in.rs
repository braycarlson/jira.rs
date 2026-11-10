#[derive(Clone, Copy, Debug)]
pub struct GlobalConfigurationIn;

impl GlobalConfigurationIn {
    pub const ALLOW_OTHER_OBJECT_SCHEMA: crate::Field<Self, crate::Boolean> =
        crate::Field::new("allowOtherObjectSchema");

    pub const QUICK_CREATE_OBJECTS: crate::Field<Self, crate::Boolean> =
        crate::Field::new("quickCreateObjects");

    pub const VALIDATE_QUICK_CREATE: crate::Field<Self, crate::Boolean> =
        crate::Field::new("validateQuickCreate");
}
