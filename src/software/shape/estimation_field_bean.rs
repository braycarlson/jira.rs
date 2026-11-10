#[derive(Clone, Copy, Debug)]
pub struct EstimationFieldBean;

impl EstimationFieldBean {
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");
}
