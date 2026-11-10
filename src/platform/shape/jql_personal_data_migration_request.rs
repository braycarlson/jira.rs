#[derive(Clone, Copy, Debug)]
pub struct JQLPersonalDataMigrationRequest;

impl JQLPersonalDataMigrationRequest {
    pub const QUERY_STRINGS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("queryStrings");
}
