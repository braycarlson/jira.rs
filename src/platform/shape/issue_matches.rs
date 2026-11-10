#[derive(Clone, Copy, Debug)]
pub struct IssueMatches;

impl IssueMatches {
    pub const MATCHES: crate::Field<Self, crate::List<crate::Object<super::IssueMatchesForJQL>>> =
        crate::Field::new("matches");
}
