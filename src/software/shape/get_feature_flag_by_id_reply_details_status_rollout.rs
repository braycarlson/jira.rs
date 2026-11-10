#[derive(Clone, Copy, Debug)]
pub struct GetFeatureFlagByIDReplyDetailsStatusRollout;

impl GetFeatureFlagByIDReplyDetailsStatusRollout {
    pub const PERCENTAGE: crate::Field<Self, crate::Number> = crate::Field::new("percentage");
    pub const RULES: crate::Field<Self, crate::Integer> = crate::Field::new("rules");
    pub const TEXT: crate::Field<Self, crate::Text> = crate::Field::new("text");
}
