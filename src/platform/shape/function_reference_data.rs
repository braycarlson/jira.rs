#[derive(Clone, Copy, Debug)]
pub struct FunctionReferenceData;

impl FunctionReferenceData {
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const IS_LIST: crate::Field<Self, crate::Text> = crate::Field::new("isList");

    pub const SUPPORTS_LIST_AND_SINGLE_VALUE_OPERATORS: crate::Field<Self, crate::Text> =
        crate::Field::new("supportsListAndSingleValueOperators");

    pub const TYPES: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("types");
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
