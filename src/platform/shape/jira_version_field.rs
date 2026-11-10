#[derive(Clone, Copy, Debug)]
pub struct JiraVersionField;

impl JiraVersionField {
    pub const VERSION_ID: crate::Field<Self, crate::Text> = crate::Field::new("versionId");
}
