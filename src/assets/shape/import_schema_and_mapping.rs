#[derive(Clone, Copy, Debug)]
pub struct ImportSchemaAndMapping;

impl ImportSchemaAndMapping {
    pub const MAPPING: crate::Field<Self, crate::Any> = crate::Field::new("mapping");
    pub const SCHEMA: crate::Field<Self, crate::Any> = crate::Field::new("schema");
}
