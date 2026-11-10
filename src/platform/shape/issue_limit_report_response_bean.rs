#[derive(Clone, Copy, Debug)]
pub struct IssueLimitReportResponseBean;

impl IssueLimitReportResponseBean {
    pub const ENTITIES_BREACHING_LIMIT: crate::Field<
        Self,
        crate::Map<crate::Map<crate::List<crate::Integer>>>,
    > = crate::Field::new("entitiesBreachingLimit");

    pub const ISSUES_APPROACHING_LIMIT: crate::Field<Self, crate::Map<crate::Map<crate::Integer>>> =
        crate::Field::new("issuesApproachingLimit");

    pub const ISSUES_BREACHING_LIMIT: crate::Field<Self, crate::Map<crate::Map<crate::Integer>>> =
        crate::Field::new("issuesBreachingLimit");

    pub const LIMITS: crate::Field<Self, crate::Map<crate::Integer>> = crate::Field::new("limits");
}
