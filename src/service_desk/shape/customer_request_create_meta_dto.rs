#[derive(Clone, Copy, Debug)]
pub struct CustomerRequestCreateMetaDTO;

impl CustomerRequestCreateMetaDTO {
    pub const CAN_ADD_REQUEST_PARTICIPANTS: crate::Field<Self, crate::Boolean> =
        crate::Field::new("canAddRequestParticipants");

    pub const CAN_RAISE_ON_BEHALF_OF: crate::Field<Self, crate::Boolean> =
        crate::Field::new("canRaiseOnBehalfOf");

    pub const REQUEST_TYPE_FIELDS: crate::Field<
        Self,
        crate::List<crate::Object<super::RequestTypeFieldDTO>>,
    > = crate::Field::new("requestTypeFields");
}
