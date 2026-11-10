#[derive(Clone, Copy, Debug)]
pub struct IssuesAndJQLQueries;

impl IssuesAndJQLQueries {
    pub const ISSUE_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("issueIds");

    pub const JQLS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("jqls");
}
