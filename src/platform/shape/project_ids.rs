#[derive(Clone, Copy, Debug)]
pub struct ProjectIDs;

impl ProjectIDs {
    pub const PROJECT_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("projectIds");
}
