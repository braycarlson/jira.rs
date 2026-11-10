#[derive(Clone, Copy, Debug)]
pub struct IssueLayoutPayload;

impl IssueLayoutPayload {
    pub const CONTAINER_ID: crate::Field<
        Self,
        crate::Object<super::ProjectCreateResourceIdentifier>,
    > = crate::Field::new("containerId");

    pub const ISSUE_LAYOUT_TYPE: crate::Field<Self, crate::Text> =
        crate::Field::new("issueLayoutType");

    pub const ITEMS: crate::Field<Self, crate::List<crate::Object<super::IssueLayoutItemPayload>>> =
        crate::Field::new("items");

    pub const PCRI: crate::Field<Self, crate::Object<super::ProjectCreateResourceIdentifier>> =
        crate::Field::new("pcri");
}
