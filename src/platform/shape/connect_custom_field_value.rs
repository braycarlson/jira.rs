#[derive(Clone, Copy, Debug)]
pub struct ConnectCustomFieldValue;

impl ConnectCustomFieldValue {
    pub const FIELD_ID: crate::Field<Self, crate::Integer> = crate::Field::new("fieldID");
    pub const ISSUE_ID: crate::Field<Self, crate::Integer> = crate::Field::new("issueID");
    pub const NUMBER: crate::Field<Self, crate::Number> = crate::Field::new("number");
    pub const OPTION_ID: crate::Field<Self, crate::Text> = crate::Field::new("optionID");
    pub const RICH_TEXT: crate::Field<Self, crate::Text> = crate::Field::new("richText");
    pub const STRING: crate::Field<Self, crate::Text> = crate::Field::new("string");
    pub const TEXT: crate::Field<Self, crate::Text> = crate::Field::new("text");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("_type");
}
