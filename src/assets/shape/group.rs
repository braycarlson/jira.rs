#[derive(Clone, Copy, Debug)]
pub struct Group;

impl Group {
    pub const AVATAR_URL: crate::Field<Self, crate::Text> = crate::Field::new("avatarUrl");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
