#[derive(Clone, Copy, Debug)]
pub struct FieldAssociationSchemeFieldSearchResult;

impl FieldAssociationSchemeFieldSearchResult {
    pub const ALLOWED_OPERATIONS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("allowedOperations");

    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");

    pub const PARAMETERS: crate::Field<Self, crate::Object<super::SearchResultFieldParameters>> =
        crate::Field::new("parameters");

    pub const RESTRICTED_TO_WORK_TYPES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("restrictedToWorkTypes");

    pub const WORK_TYPE_PARAMETERS: crate::Field<
        Self,
        crate::List<crate::Object<super::SearchResultWorkTypeParameters>>,
    > = crate::Field::new("workTypeParameters");
}
