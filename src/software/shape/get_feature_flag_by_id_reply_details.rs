#[derive(Clone, Copy, Debug)]
pub struct GetFeatureFlagByIDReplyDetails;

impl GetFeatureFlagByIDReplyDetails {
    pub const ENVIRONMENT: crate::Field<
        Self,
        crate::Object<super::GetFeatureFlagByIDReplyDetailsEnvironment>,
    > = crate::Field::new("environment");

    pub const STATUS: crate::Field<
        Self,
        crate::Object<super::GetFeatureFlagByIDReplyDetailsStatus>,
    > = crate::Field::new("status");

    pub const UPDATED_LAST: crate::Field<Self, crate::Text> = crate::Field::new("lastUpdated");
    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
}
