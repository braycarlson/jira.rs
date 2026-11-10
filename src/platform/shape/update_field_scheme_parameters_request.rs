#[derive(Clone, Copy, Debug)]
pub struct UpdateFieldSchemeParametersRequest;

impl UpdateFieldSchemeParametersRequest {
    pub const PARAMETERS: crate::Field<Self, crate::Object<super::FieldsSchemeItemParameter>> =
        crate::Field::new("parameters");

    pub const SCHEME_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("schemeIds");

    pub const WORK_TYPE_PARAMETERS: crate::Field<
        Self,
        crate::List<crate::Object<super::FieldsSchemeItemWorkTypeParameter>>,
    > = crate::Field::new("workTypeParameters");
}
