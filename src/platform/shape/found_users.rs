#[derive(Clone, Copy, Debug)]
pub struct FoundUsers;

impl FoundUsers {
    pub const HEADER: crate::Field<Self, crate::Text> = crate::Field::new("header");
    pub const TOTAL: crate::Field<Self, crate::Integer> = crate::Field::new("total");

    pub const USERS: crate::Field<Self, crate::List<crate::Object<super::UserPickerUser>>> =
        crate::Field::new("users");
}
