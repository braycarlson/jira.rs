#[derive(Clone, Copy, Debug)]
pub struct Priority;

impl Priority {
    pub const AVATAR_ID: crate::Field<Self, crate::Integer> = crate::Field::new("avatarId");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ICON_URL: crate::Field<Self, crate::Text> = crate::Field::new("iconUrl");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const IS_DEFAULT: crate::Field<Self, crate::Boolean> = crate::Field::new("isDefault");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const SCHEMES: crate::Field<Self, crate::Object<super::ExpandPrioritySchemePage>> =
        crate::Field::new("schemes");

    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
    pub const STATUS_COLOR: crate::Field<Self, crate::Text> = crate::Field::new("statusColor");
}
