#[derive(Clone, Copy, Debug)]
pub struct GetConfigurationReplyRanking;

impl GetConfigurationReplyRanking {
    pub const RANK_CUSTOM_FIELD_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("rankCustomFieldId");
}
