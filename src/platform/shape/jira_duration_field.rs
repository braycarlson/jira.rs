#[derive(Clone, Copy, Debug)]
pub struct JiraDurationField;

impl JiraDurationField {
    pub const ORIGINAL_ESTIMATE_FIELD: crate::Field<Self, crate::Text> =
        crate::Field::new("originalEstimateField");
}
