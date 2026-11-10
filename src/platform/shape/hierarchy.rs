#[derive(Clone, Copy, Debug)]
pub struct Hierarchy;

impl Hierarchy {
    pub const BASE_LEVEL_ID: crate::Field<Self, crate::Integer> = crate::Field::new("baseLevelId");

    pub const LEVELS: crate::Field<
        Self,
        crate::List<crate::Object<super::SimplifiedHierarchyLevel>>,
    > = crate::Field::new("levels");
}
