#[derive(Clone, Copy, Debug)]
pub struct ProjectIssueSecurityLevels;

impl ProjectIssueSecurityLevels {
    pub const LEVELS: crate::Field<Self, crate::List<crate::Object<super::SecurityLevel>>> =
        crate::Field::new("levels");
}
