#[derive(Clone, Copy, Debug)]
pub struct JQLReferenceData;

impl JQLReferenceData {
    pub const JQL_RESERVED_WORDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("jqlReservedWords");

    pub const VISIBLE_FIELD_NAMES: crate::Field<
        Self,
        crate::List<crate::Object<super::FieldReferenceData>>,
    > = crate::Field::new("visibleFieldNames");

    pub const VISIBLE_FUNCTION_NAMES: crate::Field<
        Self,
        crate::List<crate::Object<super::FunctionReferenceData>>,
    > = crate::Field::new("visibleFunctionNames");
}
