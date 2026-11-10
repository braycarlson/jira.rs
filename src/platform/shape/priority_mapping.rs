#[derive(Clone, Copy, Debug)]
pub struct PriorityMapping;

impl PriorityMapping {
    pub const IN_VALUE: crate::Field<Self, crate::Map<crate::Integer>> = crate::Field::new("in");
    pub const OUT: crate::Field<Self, crate::Map<crate::Integer>> = crate::Field::new("out");
}
