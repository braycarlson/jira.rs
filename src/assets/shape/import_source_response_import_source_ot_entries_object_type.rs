#[derive(Clone, Copy, Debug)]
pub struct ImportSourceResponseImportSourceOTEntriesObjectType;

impl ImportSourceResponseImportSourceOTEntriesObjectType {
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
