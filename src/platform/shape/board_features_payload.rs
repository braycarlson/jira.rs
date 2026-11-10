#[derive(Clone, Copy, Debug)]
pub struct BoardFeaturesPayload;

impl BoardFeaturesPayload {
    pub const BOARD_FEATURES: crate::Field<
        Self,
        crate::Map<crate::List<crate::Object<super::BoardFeaturePayload>>>,
    > = crate::Field::new("boardFeatures");
}
