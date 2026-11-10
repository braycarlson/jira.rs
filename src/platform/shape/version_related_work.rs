#[derive(Clone, Copy, Debug)]
pub struct VersionRelatedWork;

impl VersionRelatedWork {
    pub const CATEGORY: crate::Field<Self, crate::Text> = crate::Field::new("category");
    pub const ISSUE_ID: crate::Field<Self, crate::Integer> = crate::Field::new("issueId");
    pub const RELATED_WORK_ID: crate::Field<Self, crate::Text> = crate::Field::new("relatedWorkId");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");
    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
}
