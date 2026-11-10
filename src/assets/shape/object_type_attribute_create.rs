#[derive(Clone, Copy, Debug)]
pub struct ObjectTypeAttributeCreate;

impl ObjectTypeAttributeCreate {
    pub const ADDITIONAL_VALUE: crate::Field<Self, crate::Text> =
        crate::Field::new("additionalValue");

    pub const CARDINALITY_MAXIMUM: crate::Field<Self, crate::Integer> =
        crate::Field::new("maximumCardinality");

    pub const CARDINALITY_MINIMUM: crate::Field<Self, crate::Integer> =
        crate::Field::new("minimumCardinality");

    pub const DEFAULT_TYPE_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("defaultTypeId");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const HIDDEN: crate::Field<Self, crate::Boolean> = crate::Field::new("hidden");

    pub const INCLUDE_CHILD_OBJECT_TYPES: crate::Field<Self, crate::Boolean> =
        crate::Field::new("includeChildObjectTypes");

    pub const IQL: crate::Field<Self, crate::Text> = crate::Field::new("iql");
    pub const LABEL: crate::Field<Self, crate::Boolean> = crate::Field::new("label");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const OPTIONS: crate::Field<Self, crate::Text> = crate::Field::new("options");
    pub const QL_QUERY: crate::Field<Self, crate::Text> = crate::Field::new("qlQuery");

    pub const REGEX_VALIDATION: crate::Field<Self, crate::Text> =
        crate::Field::new("regexValidation");

    pub const SUFFIX: crate::Field<Self, crate::Text> = crate::Field::new("suffix");
    pub const SUMMABLE: crate::Field<Self, crate::Boolean> = crate::Field::new("summable");
    pub const TYPE: crate::Field<Self, crate::Integer> = crate::Field::new("type");
    pub const TYPE_VALUE: crate::Field<Self, crate::Text> = crate::Field::new("typeValue");

    pub const TYPE_VALUE_MULTI: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("typeValueMulti");

    pub const UNIQUE_ATTRIBUTE: crate::Field<Self, crate::Boolean> =
        crate::Field::new("uniqueAttribute");
}
