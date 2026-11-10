#[derive(Clone, Copy, Debug)]
pub struct IssueTypeProjectCreatePayload;

impl IssueTypeProjectCreatePayload {
    pub const ISSUE_TYPES: crate::Field<Self, crate::List<crate::Object<super::IssueTypePayload>>> =
        crate::Field::new("issueTypes");

    pub const ISSUE_TYPE_HIERARCHY: crate::Field<
        Self,
        crate::List<crate::Object<super::IssueTypeHierarchyPayload>>,
    > = crate::Field::new("issueTypeHierarchy");

    pub const ISSUE_TYPE_SCHEME: crate::Field<Self, crate::Object<super::IssueTypeSchemePayload>> =
        crate::Field::new("issueTypeScheme");
}
