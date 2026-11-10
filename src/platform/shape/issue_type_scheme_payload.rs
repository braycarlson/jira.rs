#[derive(Clone, Copy, Debug)]
pub struct IssueTypeSchemePayload;

impl IssueTypeSchemePayload {
    pub const DEFAULT_ISSUE_TYPE_ID: crate::Field<
        Self,
        crate::Object<super::ProjectCreateResourceIdentifier>,
    > = crate::Field::new("defaultIssueTypeId");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const ISSUE_TYPE_IDS: crate::Field<
        Self,
        crate::List<crate::Object<super::ProjectCreateResourceIdentifier>>,
    > = crate::Field::new("issueTypeIds");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const PCRI: crate::Field<Self, crate::Object<super::ProjectCreateResourceIdentifier>> =
        crate::Field::new("pcri");
}
