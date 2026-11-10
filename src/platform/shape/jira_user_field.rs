#[derive(Clone, Copy, Debug)]
pub struct JiraUserField;

impl JiraUserField {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
}
