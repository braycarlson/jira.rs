#[derive(Clone, Copy, Debug)]
pub struct CreateIssueSecuritySchemeDetails;

impl CreateIssueSecuritySchemeDetails {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const LEVELS: crate::Field<
        Self,
        crate::List<crate::Object<super::SecuritySchemeLevelBean>>,
    > = crate::Field::new("levels");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
