#[derive(Clone, Copy, Debug)]
pub struct ReferenceTypeCdmType;

impl ReferenceTypeCdmType {
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const OPINIONATED: crate::Field<Self, crate::Boolean> = crate::Field::new("opinionated");
    pub const VERSION: crate::Field<Self, crate::Integer> = crate::Field::new("version");
}
