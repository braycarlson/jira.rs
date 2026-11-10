#[derive(Clone, Copy, Debug)]
pub struct CommentCreateDTO;

impl CommentCreateDTO {
    pub const BODY: crate::Field<Self, crate::Text> = crate::Field::new("body");
    pub const PUBLIC: crate::Field<Self, crate::Boolean> = crate::Field::new("public");
}
