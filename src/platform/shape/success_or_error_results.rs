#[derive(Clone, Copy, Debug)]
pub struct SuccessOrErrorResults;

impl SuccessOrErrorResults {
    pub const ERROR: crate::Field<Self, crate::Object<super::RemoveFieldParametersResultError>> =
        crate::Field::new("error");

    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");
    pub const SCHEME_ID: crate::Field<Self, crate::Integer> = crate::Field::new("schemeId");
    pub const SUCCESS: crate::Field<Self, crate::Boolean> = crate::Field::new("success");

    pub const WORK_TYPE_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("workTypeIds");
}
