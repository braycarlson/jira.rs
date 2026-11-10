#[derive(Clone, Copy, Debug)]
pub struct IssueTypeHierarchyPayload;

impl IssueTypeHierarchyPayload {
    pub const HIERARCHY_LEVEL: crate::Field<Self, crate::Integer> =
        crate::Field::new("hierarchyLevel");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const ON_CONFLICT: crate::Field<Self, crate::Text> = crate::Field::new("onConflict");

    pub const PCRI: crate::Field<Self, crate::Object<super::ProjectCreateResourceIdentifier>> =
        crate::Field::new("pcri");
}
