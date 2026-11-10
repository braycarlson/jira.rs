#[derive(Clone, Copy, Debug)]
pub struct BulkIssuePropertyUpdateRequest;

impl BulkIssuePropertyUpdateRequest {
    pub const EXPRESSION: crate::Field<Self, crate::Text> = crate::Field::new("expression");

    pub const FILTER: crate::Field<Self, crate::Object<super::IssueFilterForBulkPropertySet>> =
        crate::Field::new("filter");

    pub const VALUE: crate::Field<Self, crate::Any> = crate::Field::new("value");
}
