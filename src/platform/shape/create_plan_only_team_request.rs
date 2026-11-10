#[derive(Clone, Copy, Debug)]
pub struct CreatePlanOnlyTeamRequest;

impl CreatePlanOnlyTeamRequest {
    pub const CAPACITY: crate::Field<Self, crate::Number> = crate::Field::new("capacity");

    pub const ISSUE_SOURCE_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("issueSourceId");

    pub const MEMBER_ACCOUNT_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("memberAccountIds");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const PLANNING_STYLE: crate::Field<Self, crate::Text> = crate::Field::new("planningStyle");
    pub const SPRINT_LENGTH: crate::Field<Self, crate::Integer> = crate::Field::new("sprintLength");
}
