#[derive(Clone, Copy, Debug)]
pub struct BoardConfigBeanRanking;

impl BoardConfigBeanRanking {
    pub const RANK_CUSTOM_FIELD_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("rankCustomFieldId");
}
