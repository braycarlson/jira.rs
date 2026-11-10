#[derive(Clone, Copy, Debug)]
pub struct AdditionalCommentDTO;

impl AdditionalCommentDTO {
    pub const BODY: crate::Field<Self, crate::Text> = crate::Field::new("body");
}
