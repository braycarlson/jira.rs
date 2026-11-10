#[derive(Clone, Copy, Debug)]
pub struct FoundGroups;

impl FoundGroups {
    pub const GROUPS: crate::Field<Self, crate::List<crate::Object<super::FoundGroup>>> =
        crate::Field::new("groups");

    pub const HEADER: crate::Field<Self, crate::Text> = crate::Field::new("header");
    pub const TOTAL: crate::Field<Self, crate::Integer> = crate::Field::new("total");
}
