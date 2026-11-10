#[derive(Clone, Copy, Debug)]
pub struct ToggleFeaturesReply;

impl ToggleFeaturesReply {
    pub const FEATURES: crate::Field<
        Self,
        crate::List<crate::Object<super::ToggleFeaturesReplyFeatures>>,
    > = crate::Field::new("features");
}
