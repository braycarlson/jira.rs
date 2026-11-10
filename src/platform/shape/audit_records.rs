#[derive(Clone, Copy, Debug)]
pub struct AuditRecords;

impl AuditRecords {
    pub const LIMIT: crate::Field<Self, crate::Integer> = crate::Field::new("limit");
    pub const OFFSET: crate::Field<Self, crate::Integer> = crate::Field::new("offset");

    pub const RECORDS: crate::Field<Self, crate::List<crate::Object<super::AuditRecordBean>>> =
        crate::Field::new("records");

    pub const TOTAL: crate::Field<Self, crate::Integer> = crate::Field::new("total");
}
