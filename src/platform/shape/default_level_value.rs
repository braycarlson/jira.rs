#[derive(Clone, Copy, Debug)]
pub struct DefaultLevelValue;

impl DefaultLevelValue {
    pub const DEFAULT_LEVEL_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("defaultLevelId");

    pub const ISSUE_SECURITY_SCHEME_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("issueSecuritySchemeId");
}
