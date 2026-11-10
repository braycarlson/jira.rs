#[derive(Clone, Copy, Debug)]
pub struct ObjectListResult;

impl ObjectListResult {
    pub const ABSTRACT_OBJECT_TYPE: crate::Field<Self, crate::Boolean> =
        crate::Field::new("abstractObjectType");

    pub const CONVERSION_POSSIBLE: crate::Field<Self, crate::Boolean> =
        crate::Field::new("conversionPossible");

    pub const FILTERS: crate::Field<Self, crate::Any> = crate::Field::new("filters");

    pub const FILTER_COUNT_TOTAL: crate::Field<Self, crate::Integer> =
        crate::Field::new("totalFilterCount");

    pub const HAS_MORE_RESULTS: crate::Field<Self, crate::Boolean> =
        crate::Field::new("hasMoreResults");

    pub const INHERITANCE_TREE: crate::Field<Self, crate::Any> =
        crate::Field::new("inheritanceTree");

    pub const IQL: crate::Field<Self, crate::Text> = crate::Field::new("iql");

    pub const IQL_SEARCH_RESULT: crate::Field<Self, crate::Boolean> =
        crate::Field::new("iqlSearchResult");

    pub const MATCHED_FILTER_VALUES: crate::Field<Self, crate::Any> =
        crate::Field::new("matchedFilterValues");

    pub const OBJECT_ENTRIES: crate::Field<Self, crate::List<crate::Object<super::Object>>> =
        crate::Field::new("objectEntries");

    pub const OBJECT_TYPE_ATTRIBUTES: crate::Field<
        Self,
        crate::List<crate::Object<super::ObjectTypeAttribute>>,
    > = crate::Field::new("objectTypeAttributes");

    pub const OBJECT_TYPE_ID: crate::Field<Self, crate::Text> = crate::Field::new("objectTypeId");

    pub const OBJECT_TYPE_IS_INHERITED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("objectTypeIsInherited");

    pub const ORDER_BY_TYPE_ATTRIBUTE_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("orderByTypeAttrId");

    pub const ORDER_WAY: crate::Field<Self, crate::Text> = crate::Field::new("orderWay");
    pub const PAGE_NUMBER: crate::Field<Self, crate::Integer> = crate::Field::new("pageNumber");

    pub const PAGE_OBJECT_SIZE: crate::Field<Self, crate::Integer> =
        crate::Field::new("pageObjectSize");

    pub const PAGE_SIZE: crate::Field<Self, crate::Integer> = crate::Field::new("pageSize");
    pub const QL_QUERY: crate::Field<Self, crate::Text> = crate::Field::new("qlQuery");

    pub const QL_QUERY_SEARCH_RESULT: crate::Field<Self, crate::Boolean> =
        crate::Field::new("qlQuerySearchResult");

    pub const START_INDEX: crate::Field<Self, crate::Integer> = crate::Field::new("startIndex");
    pub const TO_INDEX: crate::Field<Self, crate::Integer> = crate::Field::new("toIndex");
}
