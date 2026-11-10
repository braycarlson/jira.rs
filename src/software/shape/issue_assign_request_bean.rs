#[derive(Clone, Copy, Debug)]
pub struct IssueAssignRequestBean;

impl IssueAssignRequestBean {
    pub const ISSUES: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("issues");
}
