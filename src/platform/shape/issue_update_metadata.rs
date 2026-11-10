#[derive(Clone, Copy, Debug)]
pub struct IssueUpdateMetadata;

impl IssueUpdateMetadata {
    pub const FIELDS: crate::Field<Self, crate::Map<crate::Object<super::FieldMetadata>>> =
        crate::Field::new("fields");
}
