#[derive(Clone, Copy, Debug)]
pub struct ActorsMap;

impl ActorsMap {
    pub const GROUP: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("group");
    pub const GROUP_ID: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("groupId");
    pub const USER: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("user");
}
