#[derive(Clone, Copy, Debug)]
pub struct ObjectIn;

impl ObjectIn {
    pub const ATTRIBUTES: crate::Field<Self, crate::List<crate::Object<super::ObjectAttributeIn>>> =
        crate::Field::new("attributes");

    pub const AVATAR_UUID: crate::Field<Self, crate::Text> = crate::Field::new("avatarUUID");
    pub const HAS_AVATAR: crate::Field<Self, crate::Boolean> = crate::Field::new("hasAvatar");
    pub const OBJECT_TYPE_ID: crate::Field<Self, crate::Text> = crate::Field::new("objectTypeId");
}
