#[derive(Clone, Copy, Debug)]
pub struct EpicRankRequestBean;

impl EpicRankRequestBean {
    pub const RANK_AFTER_EPIC: crate::Field<Self, crate::Text> = crate::Field::new("rankAfterEpic");

    pub const RANK_BEFORE_EPIC: crate::Field<Self, crate::Text> =
        crate::Field::new("rankBeforeEpic");

    pub const RANK_CUSTOM_FIELD_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("rankCustomFieldId");
}
