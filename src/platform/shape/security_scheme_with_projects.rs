#[derive(Clone, Copy, Debug)]
pub struct SecuritySchemeWithProjects;

impl SecuritySchemeWithProjects {
    pub const DEFAULT_LEVEL: crate::Field<Self, crate::Integer> = crate::Field::new("defaultLevel");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const PROJECT_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("projectIds");

    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
