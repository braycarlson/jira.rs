#[derive(Clone, Copy, Debug)]
pub struct IssueTypeScreenSchemeItem;

impl IssueTypeScreenSchemeItem {
    pub const ISSUE_TYPE_ID: crate::Field<Self, crate::Text> = crate::Field::new("issueTypeId");

    pub const ISSUE_TYPE_SCREEN_SCHEME_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("issueTypeScreenSchemeId");

    pub const SCREEN_SCHEME_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("screenSchemeId");
}
