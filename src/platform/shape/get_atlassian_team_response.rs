#[derive(Clone, Copy, Debug)]
pub struct GetAtlassianTeamResponse;

impl GetAtlassianTeamResponse {
    pub const CAPACITY: crate::Field<Self, crate::Number> = crate::Field::new("capacity");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const ISSUE_SOURCE_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("issueSourceId");

    pub const PLANNING_STYLE: crate::Field<Self, crate::Text> = crate::Field::new("planningStyle");
    pub const SPRINT_LENGTH: crate::Field<Self, crate::Integer> = crate::Field::new("sprintLength");
}
