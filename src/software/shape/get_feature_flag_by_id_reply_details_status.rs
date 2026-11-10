#[derive(Clone, Copy, Debug)]
pub struct GetFeatureFlagByIDReplyDetailsStatus;

impl GetFeatureFlagByIDReplyDetailsStatus {
    pub const DEFAULT_VALUE: crate::Field<Self, crate::Text> = crate::Field::new("defaultValue");
    pub const ENABLED: crate::Field<Self, crate::Boolean> = crate::Field::new("enabled");

    pub const ROLLOUT: crate::Field<
        Self,
        crate::Object<super::GetFeatureFlagByIDReplyDetailsStatusRollout>,
    > = crate::Field::new("rollout");
}
