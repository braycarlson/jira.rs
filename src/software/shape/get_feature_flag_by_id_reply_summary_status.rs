#[derive(Clone, Copy, Debug)]
pub struct GetFeatureFlagByIDReplySummaryStatus;

impl GetFeatureFlagByIDReplySummaryStatus {
    pub const DEFAULT_VALUE: crate::Field<Self, crate::Text> = crate::Field::new("defaultValue");
    pub const ENABLED: crate::Field<Self, crate::Boolean> = crate::Field::new("enabled");

    pub const ROLLOUT: crate::Field<
        Self,
        crate::Object<super::GetFeatureFlagByIDReplySummaryStatusRollout>,
    > = crate::Field::new("rollout");
}
