#[derive(Clone, Copy, Debug)]
pub struct GetPlanResponseForPage;

impl GetPlanResponseForPage {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const ISSUE_SOURCES: crate::Field<
        Self,
        crate::List<crate::Object<super::GetIssueSourceResponse>>,
    > = crate::Field::new("issueSources");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const SCENARIO_ID: crate::Field<Self, crate::Text> = crate::Field::new("scenarioId");
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
}
