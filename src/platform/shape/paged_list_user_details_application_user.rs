#[derive(Clone, Copy, Debug)]
pub struct PagedListUserDetailsApplicationUser;

impl PagedListUserDetailsApplicationUser {
    pub const END_INDEX: crate::Field<Self, crate::Integer> = crate::Field::new("end-index");

    pub const ITEMS: crate::Field<Self, crate::List<crate::Object<super::UserDetails>>> =
        crate::Field::new("items");

    pub const RESULTS_MAX: crate::Field<Self, crate::Integer> = crate::Field::new("max-results");
    pub const SIZE: crate::Field<Self, crate::Integer> = crate::Field::new("size");
    pub const START_INDEX: crate::Field<Self, crate::Integer> = crate::Field::new("start-index");
}
