#[derive(Clone, Copy, Debug)]
pub struct GetFeatureFlagByIDReplySummary;

impl GetFeatureFlagByIDReplySummary {
    pub const STATUS: crate::Field<
        Self,
        crate::Object<super::GetFeatureFlagByIDReplySummaryStatus>,
    > = crate::Field::new("status");

    pub const UPDATED_LAST: crate::Field<Self, crate::Text> = crate::Field::new("lastUpdated");
    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
}
