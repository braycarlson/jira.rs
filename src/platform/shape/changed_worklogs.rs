#[derive(Clone, Copy, Debug)]
pub struct ChangedWorklogs;

impl ChangedWorklogs {
    pub const PAGE_LAST: crate::Field<Self, crate::Boolean> = crate::Field::new("lastPage");
    pub const PAGE_NEXT: crate::Field<Self, crate::Text> = crate::Field::new("nextPage");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
    pub const SINCE: crate::Field<Self, crate::Integer> = crate::Field::new("since");
    pub const UNTIL: crate::Field<Self, crate::Integer> = crate::Field::new("until");

    pub const VALUES: crate::Field<Self, crate::List<crate::Object<super::ChangedWorklog>>> =
        crate::Field::new("values");
}
