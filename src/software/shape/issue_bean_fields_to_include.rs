#[derive(Clone, Copy, Debug)]
pub struct IssueBeanFieldsToInclude;

impl IssueBeanFieldsToInclude {
    pub const ACTUALLY_INCLUDED: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("actuallyIncluded");

    pub const EXCLUDED: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("excluded");

    pub const INCLUDED: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("included");
}
