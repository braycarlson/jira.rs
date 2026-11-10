#[derive(Clone, Copy, Debug)]
pub struct IssueCommentListRequestBean;

impl IssueCommentListRequestBean {
    pub const IDS: crate::Field<Self, crate::List<crate::Integer>> = crate::Field::new("ids");
}
