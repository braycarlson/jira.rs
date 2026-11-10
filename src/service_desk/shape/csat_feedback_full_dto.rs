#[derive(Clone, Copy, Debug)]
pub struct CSATFeedbackFullDTO;

impl CSATFeedbackFullDTO {
    pub const COMMENT: crate::Field<Self, crate::Object<super::AdditionalCommentDTO>> =
        crate::Field::new("comment");

    pub const RATING: crate::Field<Self, crate::Integer> = crate::Field::new("rating");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
