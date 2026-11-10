#[derive(Clone, Copy, Debug)]
pub struct IssueTypeScreenSchemePayload;

impl IssueTypeScreenSchemePayload {
    pub const DEFAULT_SCREEN_SCHEME: crate::Field<
        Self,
        crate::Object<super::ProjectCreateResourceIdentifier>,
    > = crate::Field::new("defaultScreenScheme");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const EXPLICIT_MAPPINGS: crate::Field<
        Self,
        crate::Map<crate::Object<super::ProjectCreateResourceIdentifier>>,
    > = crate::Field::new("explicitMappings");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const PCRI: crate::Field<Self, crate::Object<super::ProjectCreateResourceIdentifier>> =
        crate::Field::new("pcri");
}
