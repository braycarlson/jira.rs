#[derive(Clone, Copy, Debug)]
pub struct ContainerForProjectFeatures;

impl ContainerForProjectFeatures {
    pub const FEATURES: crate::Field<Self, crate::List<crate::Object<super::ProjectFeature>>> =
        crate::Field::new("features");
}
