#[derive(Clone, Copy, Debug)]
pub struct Group;

impl Group {
    pub const EXPAND: crate::Field<Self, crate::Text> = crate::Field::new("expand");
    pub const GROUP_ID: crate::Field<Self, crate::Text> = crate::Field::new("groupId");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");

    pub const USERS: crate::Field<Self, crate::Object<super::PagedListUserDetailsApplicationUser>> =
        crate::Field::new("users");
}
