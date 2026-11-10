#[derive(Clone, Copy, Debug)]
pub struct JexpEvaluateCtxIssues;

impl JexpEvaluateCtxIssues {
    pub const JQL: crate::Field<Self, crate::Object<super::JexpEvaluateCtxJQLIssues>> =
        crate::Field::new("jql");
}
