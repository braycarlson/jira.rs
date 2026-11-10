#[derive(Clone, Copy, Debug)]
pub struct BulkAgentSessionsResponse;

impl BulkAgentSessionsResponse {
    pub const ALREADY_STARTED_COUNT: crate::Field<Self, crate::Integer> =
        crate::Field::new("alreadyStartedCount");

    pub const FAILED_COUNT: crate::Field<Self, crate::Integer> = crate::Field::new("failedCount");

    pub const RATE_LIMITED_COUNT: crate::Field<Self, crate::Integer> =
        crate::Field::new("rateLimitedCount");

    pub const RESULTS: crate::Field<
        Self,
        crate::List<crate::Object<super::BulkAgentSessionOutcome>>,
    > = crate::Field::new("results");

    pub const STARTED_COUNT: crate::Field<Self, crate::Integer> = crate::Field::new("startedCount");
    pub const SUCCESS: crate::Field<Self, crate::Boolean> = crate::Field::new("success");
}
