#[derive(Clone, Copy, Debug)]
pub struct SimplifiedHierarchyLevel;

impl SimplifiedHierarchyLevel {
    pub const ABOVE_LEVEL_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("aboveLevelId");

    pub const BELOW_LEVEL_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("belowLevelId");

    pub const EXTERNAL_UUID: crate::Field<Self, crate::Text> = crate::Field::new("externalUuid");

    pub const HIERARCHY_LEVEL_NUMBER: crate::Field<Self, crate::Integer> =
        crate::Field::new("hierarchyLevelNumber");

    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");

    pub const ISSUE_TYPE_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("issueTypeIds");

    pub const LEVEL: crate::Field<Self, crate::Integer> = crate::Field::new("level");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const PROJECT_CONFIGURATION_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("projectConfigurationId");
}
