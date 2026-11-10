#[derive(Clone, Copy, Debug)]
pub struct BoardConfigBeanFilter;

impl BoardConfigBeanFilter {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
