#[derive(Clone, Copy, Debug)]
pub struct IssueFilterForBulkPropertyDelete;

impl IssueFilterForBulkPropertyDelete {
    pub const ENTITY_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("entityIds");

    pub const VALUE_CURRENT: crate::Field<Self, crate::Any> = crate::Field::new("currentValue");
}
