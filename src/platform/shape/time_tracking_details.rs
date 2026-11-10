#[derive(Clone, Copy, Debug)]
pub struct TimeTrackingDetails;

impl TimeTrackingDetails {
    pub const ORIGINAL_ESTIMATE: crate::Field<Self, crate::Text> =
        crate::Field::new("originalEstimate");

    pub const ORIGINAL_ESTIMATE_SECONDS: crate::Field<Self, crate::Integer> =
        crate::Field::new("originalEstimateSeconds");

    pub const REMAINING_ESTIMATE: crate::Field<Self, crate::Text> =
        crate::Field::new("remainingEstimate");

    pub const REMAINING_ESTIMATE_SECONDS: crate::Field<Self, crate::Integer> =
        crate::Field::new("remainingEstimateSeconds");

    pub const TIME_SPENT: crate::Field<Self, crate::Text> = crate::Field::new("timeSpent");

    pub const TIME_SPENT_SECONDS: crate::Field<Self, crate::Integer> =
        crate::Field::new("timeSpentSeconds");
}
