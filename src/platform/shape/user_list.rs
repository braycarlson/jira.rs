#[derive(Clone, Copy, Debug)]
pub struct UserList;

impl UserList {
    pub const END_INDEX: crate::Field<Self, crate::Integer> = crate::Field::new("end-index");

    pub const ITEMS: crate::Field<Self, crate::List<crate::Object<super::User>>> =
        crate::Field::new("items");

    pub const RESULTS_MAX: crate::Field<Self, crate::Integer> = crate::Field::new("max-results");
    pub const SIZE: crate::Field<Self, crate::Integer> = crate::Field::new("size");
    pub const START_INDEX: crate::Field<Self, crate::Integer> = crate::Field::new("start-index");
}
