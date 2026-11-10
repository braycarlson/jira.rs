#[derive(Clone, Copy, Debug)]
pub struct ServiceRegistry;

impl ServiceRegistry {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const ORGANIZATION_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("organizationId");

    pub const REVISION: crate::Field<Self, crate::Text> = crate::Field::new("revision");

    pub const SERVICE_TIER: crate::Field<Self, crate::Object<super::ServiceRegistryTier>> =
        crate::Field::new("serviceTier");
}
