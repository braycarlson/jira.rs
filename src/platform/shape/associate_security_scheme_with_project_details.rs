#[derive(Clone, Copy, Debug)]
pub struct AssociateSecuritySchemeWithProjectDetails;

impl AssociateSecuritySchemeWithProjectDetails {
    pub const PROJECT_ID: crate::Field<Self, crate::Text> = crate::Field::new("projectId");
    pub const SCHEME_ID: crate::Field<Self, crate::Text> = crate::Field::new("schemeId");

    pub const TO_SECURITY_LEVEL_MAPPINGS_OLD_NEW: crate::Field<
        Self,
        crate::List<crate::Object<super::OldToNewSecurityLevelMappingsBean>>,
    > = crate::Field::new("oldToNewSecurityLevelMappings");
}
