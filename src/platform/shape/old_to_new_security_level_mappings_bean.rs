#[derive(Clone, Copy, Debug)]
pub struct OldToNewSecurityLevelMappingsBean;

impl OldToNewSecurityLevelMappingsBean {
    pub const LEVEL_ID_NEW: crate::Field<Self, crate::Text> = crate::Field::new("newLevelId");
    pub const LEVEL_ID_OLD: crate::Field<Self, crate::Text> = crate::Field::new("oldLevelId");
}
