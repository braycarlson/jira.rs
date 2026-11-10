#[derive(Clone, Copy, Debug)]
pub struct UpdateFieldSchemeParametersPartialFailure;

impl UpdateFieldSchemeParametersPartialFailure {
    pub const ERROR: crate::Field<Self, crate::Text> = crate::Field::new("error");
    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");
    pub const SCHEME_ID: crate::Field<Self, crate::Integer> = crate::Field::new("schemeId");
    pub const SUCCESS: crate::Field<Self, crate::Boolean> = crate::Field::new("success");
    pub const WORK_TYPE_ID: crate::Field<Self, crate::Integer> = crate::Field::new("workTypeId");
}
