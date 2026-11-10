#[derive(Clone, Copy, Debug)]
pub struct ProjectFeatureState;

impl ProjectFeatureState {
    pub const STATE: crate::Field<Self, crate::Text> = crate::Field::new("state");
}
