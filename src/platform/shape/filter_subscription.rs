#[derive(Clone, Copy, Debug)]
pub struct FilterSubscription;

impl FilterSubscription {
    pub const GROUP: crate::Field<Self, crate::Object<super::GroupName>> =
        crate::Field::new("group");

    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const USER: crate::Field<Self, crate::Object<super::User>> = crate::Field::new("user");
}
