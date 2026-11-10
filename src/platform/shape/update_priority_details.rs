#[derive(Clone, Copy, Debug)]
pub struct UpdatePriorityDetails;

impl UpdatePriorityDetails {
    pub const AVATAR_ID: crate::Field<Self, crate::Integer> = crate::Field::new("avatarId");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ICON_URL: crate::Field<Self, crate::Text> = crate::Field::new("iconUrl");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const STATUS_COLOR: crate::Field<Self, crate::Text> = crate::Field::new("statusColor");
}
