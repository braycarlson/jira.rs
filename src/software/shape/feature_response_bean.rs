#[derive(Clone, Copy, Debug)]
pub struct FeatureResponseBean;

impl FeatureResponseBean {
    pub const FEATURES: crate::Field<
        Self,
        crate::List<crate::Object<super::FeatureResponseBeanFeatures>>,
    > = crate::Field::new("features");
}
