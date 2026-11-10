#[derive(Clone, Copy, Debug)]
pub struct CustomerRequestActionsDTO;

impl CustomerRequestActionsDTO {
    pub const ADD_ATTACHMENT: crate::Field<Self, crate::Object<super::CustomerRequestActionDTO>> =
        crate::Field::new("addAttachment");

    pub const ADD_COMMENT: crate::Field<Self, crate::Object<super::CustomerRequestActionDTO>> =
        crate::Field::new("addComment");

    pub const ADD_PARTICIPANT: crate::Field<Self, crate::Object<super::CustomerRequestActionDTO>> =
        crate::Field::new("addParticipant");

    pub const REMOVE_PARTICIPANT: crate::Field<
        Self,
        crate::Object<super::CustomerRequestActionDTO>,
    > = crate::Field::new("removeParticipant");
}
