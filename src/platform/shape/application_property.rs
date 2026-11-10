#[derive(Clone, Copy, Debug)]
pub struct ApplicationProperty;

impl ApplicationProperty {
    pub const ALLOWED_VALUES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("allowedValues");

    pub const DEFAULT_VALUE: crate::Field<Self, crate::Text> = crate::Field::new("defaultValue");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("desc");
    pub const EXAMPLE: crate::Field<Self, crate::Text> = crate::Field::new("example");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
