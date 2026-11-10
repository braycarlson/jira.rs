#[derive(Clone, Copy, Debug)]
pub struct RequestCreateDTO;

impl RequestCreateDTO {
    pub const CHANNEL: crate::Field<Self, crate::Text> = crate::Field::new("channel");
    pub const FORM: crate::Field<Self, crate::Object<super::Form>> = crate::Field::new("form");

    pub const IS_ADF_REQUEST: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isAdfRequest");

    pub const RAISE_ON_BEHALF_OF: crate::Field<Self, crate::Text> =
        crate::Field::new("raiseOnBehalfOf");

    pub const REQUEST_FIELD_VALUES: crate::Field<Self, crate::Map<crate::Any>> =
        crate::Field::new("requestFieldValues");

    pub const REQUEST_PARTICIPANTS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("requestParticipants");

    pub const REQUEST_TYPE_ID: crate::Field<Self, crate::Text> = crate::Field::new("requestTypeId");
    pub const SERVICE_DESK_ID: crate::Field<Self, crate::Text> = crate::Field::new("serviceDeskId");
}
