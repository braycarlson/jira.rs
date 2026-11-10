#[derive(Clone, Copy, Debug)]
pub struct RequiredMappingByIssueType;

impl RequiredMappingByIssueType {
    pub const ISSUE_TYPE_ID: crate::Field<Self, crate::Text> = crate::Field::new("issueTypeId");

    pub const STATUS_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("statusIds");
}
