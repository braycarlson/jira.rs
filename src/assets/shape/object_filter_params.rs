#[derive(Clone, Copy, Debug)]
pub struct ObjectFilterParams;

impl ObjectFilterParams {
    pub const ASC: crate::Field<Self, crate::Integer> = crate::Field::new("asc");

    pub const ATTRIBUTES_TO_DISPLAY: crate::Field<
        Self,
        crate::Object<super::ObjectTypeAttributesToDisplay>,
    > = crate::Field::new("attributesToDisplay");

    pub const INCLUDE_ATTRIBUTES: crate::Field<Self, crate::Boolean> =
        crate::Field::new("includeAttributes");

    pub const IQL: crate::Field<Self, crate::Text> = crate::Field::new("iql");
    pub const OBJECT_ID: crate::Field<Self, crate::Text> = crate::Field::new("objectId");

    pub const OBJECT_SCHEMA_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("objectSchemaId");

    pub const OBJECT_TYPE_ID: crate::Field<Self, crate::Text> = crate::Field::new("objectTypeId");

    pub const ORDER_BY_TYPE_ATTRIBUTE_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("orderByTypeAttrId");

    pub const PAGE: crate::Field<Self, crate::Integer> = crate::Field::new("page");
    pub const QL_QUERY: crate::Field<Self, crate::Text> = crate::Field::new("qlQuery");

    pub const RESULTS_PER_PAGE: crate::Field<Self, crate::Integer> =
        crate::Field::new("resultsPerPage");
}
