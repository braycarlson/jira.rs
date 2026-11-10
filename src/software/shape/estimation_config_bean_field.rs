#[derive(Clone, Copy, Debug)]
pub struct EstimationConfigBeanField;

impl EstimationConfigBeanField {
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");
}
