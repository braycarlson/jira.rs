#[derive(Clone, Copy, Debug)]
pub struct IssueRankRequestBean;

impl IssueRankRequestBean {
    pub const ISSUES: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("issues");

    pub const RANK_AFTER_ISSUE: crate::Field<Self, crate::Text> =
        crate::Field::new("rankAfterIssue");

    pub const RANK_BEFORE_ISSUE: crate::Field<Self, crate::Text> =
        crate::Field::new("rankBeforeIssue");

    pub const RANK_CUSTOM_FIELD_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("rankCustomFieldId");
}
