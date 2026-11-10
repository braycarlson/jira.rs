#[derive(Clone, Copy, Debug)]
pub struct CreateIssueSourceRequest;

impl CreateIssueSourceRequest {
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const VALUE: crate::Field<Self, crate::Integer> = crate::Field::new("value");
}
