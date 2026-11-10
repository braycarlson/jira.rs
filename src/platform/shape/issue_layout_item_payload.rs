#[derive(Clone, Copy, Debug)]
pub struct IssueLayoutItemPayload;

impl IssueLayoutItemPayload {
    pub const ITEM_KEY: crate::Field<Self, crate::Object<super::ProjectCreateResourceIdentifier>> =
        crate::Field::new("itemKey");

    pub const PROPERTIES: crate::Field<Self, crate::Map<crate::Any>> =
        crate::Field::new("properties");

    pub const SECTION_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("sectionType");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
