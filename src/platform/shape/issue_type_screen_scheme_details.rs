#[derive(Clone, Copy, Debug)]
pub struct IssueTypeScreenSchemeDetails;

impl IssueTypeScreenSchemeDetails {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const ISSUE_TYPE_MAPPINGS: crate::Field<
        Self,
        crate::List<crate::Object<super::IssueTypeScreenSchemeMapping>>,
    > = crate::Field::new("issueTypeMappings");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
