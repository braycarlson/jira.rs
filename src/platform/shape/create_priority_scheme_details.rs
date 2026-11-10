#[derive(Clone, Copy, Debug)]
pub struct CreatePrioritySchemeDetails;

impl CreatePrioritySchemeDetails {
    pub const DEFAULT_PRIORITY_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("defaultPriorityId");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const MAPPINGS: crate::Field<Self, crate::Object<super::PriorityMapping>> =
        crate::Field::new("mappings");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const PRIORITY_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("priorityIds");

    pub const PROJECT_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("projectIds");
}
