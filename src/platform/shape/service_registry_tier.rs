#[derive(Clone, Copy, Debug)]
pub struct ServiceRegistryTier;

impl ServiceRegistryTier {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const LEVEL: crate::Field<Self, crate::Integer> = crate::Field::new("level");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const NAME_KEY: crate::Field<Self, crate::Text> = crate::Field::new("nameKey");
}
