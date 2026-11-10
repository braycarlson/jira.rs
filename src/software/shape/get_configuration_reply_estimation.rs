#[derive(Clone, Copy, Debug)]
pub struct GetConfigurationReplyEstimation;

impl GetConfigurationReplyEstimation {
    pub const FIELD: crate::Field<
        Self,
        crate::Object<super::GetConfigurationReplyEstimationField>,
    > = crate::Field::new("field");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
