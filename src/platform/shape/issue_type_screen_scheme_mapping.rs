#[derive(Clone, Copy, Debug)]
pub struct IssueTypeScreenSchemeMapping;

impl IssueTypeScreenSchemeMapping {
    pub const ISSUE_TYPE_ID: crate::Field<Self, crate::Text> = crate::Field::new("issueTypeId");

    pub const SCREEN_SCHEME_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("screenSchemeId");
}
