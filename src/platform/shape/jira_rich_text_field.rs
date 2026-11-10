#[derive(Clone, Copy, Debug)]
pub struct JiraRichTextField;

impl JiraRichTextField {
    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");

    pub const RICH_TEXT: crate::Field<Self, crate::Object<super::JiraRichTextInput>> =
        crate::Field::new("richText");
}
