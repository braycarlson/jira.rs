#[derive(Clone, Copy, Debug)]
pub struct LicenseMetric;

impl LicenseMetric {
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
