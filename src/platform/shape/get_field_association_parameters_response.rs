#[derive(Clone, Copy, Debug)]
pub struct GetFieldAssociationParametersResponse;

impl GetFieldAssociationParametersResponse {
    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");

    pub const PARAMETERS: crate::Field<Self, crate::Object<super::FieldAssociationParameters>> =
        crate::Field::new("parameters");

    pub const WORK_TYPE_PARAMETERS: crate::Field<
        Self,
        crate::List<crate::Object<super::WorkTypeParameters>>,
    > = crate::Field::new("workTypeParameters");
}
