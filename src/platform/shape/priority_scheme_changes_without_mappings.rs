#[derive(Clone, Copy, Debug)]
pub struct PrioritySchemeChangesWithoutMappings;

impl PrioritySchemeChangesWithoutMappings {
    pub const IDS: crate::Field<Self, crate::List<crate::Integer>> = crate::Field::new("ids");
}
