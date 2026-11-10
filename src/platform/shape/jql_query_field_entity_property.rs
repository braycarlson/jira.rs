#[derive(Clone, Copy, Debug)]
pub struct JQLQueryFieldEntityProperty;

impl JQLQueryFieldEntityProperty {
    pub const ENTITY: crate::Field<Self, crate::Text> = crate::Field::new("entity");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const PATH: crate::Field<Self, crate::Text> = crate::Field::new("path");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
