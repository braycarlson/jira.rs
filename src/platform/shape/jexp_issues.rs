#[derive(Clone, Copy, Debug)]
pub struct JexpIssues;

impl JexpIssues {
    pub const JQL: crate::Field<Self, crate::Object<super::JexpJQLIssues>> =
        crate::Field::new("jql");
}
