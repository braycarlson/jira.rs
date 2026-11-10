#[derive(Clone, Copy, Debug)]
pub struct WorklogsMoveRequestBean;

impl WorklogsMoveRequestBean {
    pub const IDS: crate::Field<Self, crate::List<crate::Integer>> = crate::Field::new("ids");
    pub const ISSUE_ID_OR_KEY: crate::Field<Self, crate::Text> = crate::Field::new("issueIdOrKey");
}
