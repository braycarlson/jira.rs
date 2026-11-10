#[derive(Clone, Copy, Debug)]
pub struct CustomerTransitionExecutionDTO;

impl CustomerTransitionExecutionDTO {
    pub const ADDITIONAL_COMMENT: crate::Field<Self, crate::Object<super::AdditionalCommentDTO>> =
        crate::Field::new("additionalComment");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
