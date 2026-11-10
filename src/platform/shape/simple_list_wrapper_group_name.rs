#[derive(Clone, Copy, Debug)]
pub struct SimpleListWrapperGroupName;

impl SimpleListWrapperGroupName {
    pub const CALLBACK: crate::Field<Self, crate::Any> = crate::Field::new("callback");

    pub const ITEMS: crate::Field<Self, crate::List<crate::Object<super::GroupName>>> =
        crate::Field::new("items");

    pub const PAGING_CALLBACK: crate::Field<Self, crate::Any> = crate::Field::new("pagingCallback");
    pub const RESULTS_MAX: crate::Field<Self, crate::Integer> = crate::Field::new("max-results");
    pub const SIZE: crate::Field<Self, crate::Integer> = crate::Field::new("size");
}
