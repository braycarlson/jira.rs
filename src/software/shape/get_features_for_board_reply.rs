#[derive(Clone, Copy, Debug)]
pub struct GetFeaturesForBoardReply;

impl GetFeaturesForBoardReply {
    pub const FEATURES: crate::Field<
        Self,
        crate::List<crate::Object<super::GetFeaturesForBoardReplyFeatures>>,
    > = crate::Field::new("features");
}
