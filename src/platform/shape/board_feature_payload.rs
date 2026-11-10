#[derive(Clone, Copy, Debug)]
pub struct BoardFeaturePayload;

impl BoardFeaturePayload {
    pub const FEATURE_KEY: crate::Field<Self, crate::Text> = crate::Field::new("featureKey");
    pub const STATE: crate::Field<Self, crate::Boolean> = crate::Field::new("state");
}
