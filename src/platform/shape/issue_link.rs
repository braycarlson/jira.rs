#[derive(Clone, Copy, Debug)]
pub struct IssueLink;

impl IssueLink {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const INWARD_ISSUE: crate::Field<Self, crate::Object<super::LinkedIssue>> =
        crate::Field::new("inwardIssue");

    pub const OUTWARD_ISSUE: crate::Field<Self, crate::Object<super::LinkedIssue>> =
        crate::Field::new("outwardIssue");

    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");

    pub const TYPE: crate::Field<Self, crate::Object<super::IssueLinkType>> =
        crate::Field::new("type");
}
