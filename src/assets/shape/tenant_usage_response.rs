#[derive(Clone, Copy, Debug)]
pub struct TenantUsageResponse;

impl TenantUsageResponse {
    pub const OBJECTS_COUNT_TOTAL: crate::Field<Self, crate::Integer> =
        crate::Field::new("totalObjectsCount");

    pub const PER_SCHEMA_USAGE_INFORMATION: crate::Field<
        Self,
        crate::List<crate::Object<super::SchemaUsageInfo>>,
    > = crate::Field::new("perSchemaUsageInfo");
}
