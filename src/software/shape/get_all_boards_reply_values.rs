#[derive(Clone, Copy, Debug)]
pub struct GetAllBoardsReplyValues;

impl GetAllBoardsReplyValues {
    pub const ADMINS: crate::Field<Self, crate::Object<super::GetAllBoardsReplyValuesAdmins>> =
        crate::Field::new("admins");

    pub const CAN_EDIT: crate::Field<Self, crate::Boolean> = crate::Field::new("canEdit");
    pub const FAVOURITE: crate::Field<Self, crate::Boolean> = crate::Field::new("favourite");
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const IS_PRIVATE: crate::Field<Self, crate::Boolean> = crate::Field::new("isPrivate");

    pub const LOCATION: crate::Field<Self, crate::Object<super::GetAllBoardsReplyValuesLocation>> =
        crate::Field::new("location");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
