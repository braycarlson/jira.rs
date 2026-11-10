#[derive(Clone, Copy, Debug)]
pub struct SelfLinkDTO;

impl SelfLinkDTO {
    pub const ISSUES: crate::Field<Self, crate::Text> = crate::Field::new("issues");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
