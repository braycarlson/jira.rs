#[derive(Clone, Copy, Debug)]
pub struct IssueChangelogIDs;

impl IssueChangelogIDs {
    pub const CHANGELOG_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("changelogIds");
}
