#[derive(Clone, Copy, Debug)]
pub struct IssueFilterForBulkPropertySet;

impl IssueFilterForBulkPropertySet {
    pub const ENTITY_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("entityIds");

    pub const HAS_PROPERTY: crate::Field<Self, crate::Boolean> = crate::Field::new("hasProperty");
    pub const VALUE_CURRENT: crate::Field<Self, crate::Any> = crate::Field::new("currentValue");
}
