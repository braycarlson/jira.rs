#[derive(Clone, Copy, Debug)]
pub struct FeatureToggleRequestBean;

impl FeatureToggleRequestBean {
    pub const BOARD_ID: crate::Field<Self, crate::Integer> = crate::Field::new("boardId");
    pub const ENABLING: crate::Field<Self, crate::Boolean> = crate::Field::new("enabling");
    pub const FEATURE: crate::Field<Self, crate::Text> = crate::Field::new("feature");
}
