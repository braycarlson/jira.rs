#[derive(Clone, Copy, Debug)]
pub struct SourceDTO;

impl SourceDTO {
    pub const PAGE_ID: crate::Field<Self, crate::Text> = crate::Field::new("pageId");
    pub const SPACE_KEY: crate::Field<Self, crate::Text> = crate::Field::new("spaceKey");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
