#[derive(Clone, Copy, Debug)]
pub struct SecuritySchemePayload;

impl SecuritySchemePayload {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const PCRI: crate::Field<Self, crate::Object<super::ProjectCreateResourceIdentifier>> =
        crate::Field::new("pcri");

    pub const SECURITY_LEVELS: crate::Field<
        Self,
        crate::List<crate::Object<super::SecurityLevelPayload>>,
    > = crate::Field::new("securityLevels");
}
