#[derive(Clone, Copy, Debug)]
pub struct JiraRichTextInput;

impl JiraRichTextInput {
    pub const ADF_VALUE: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("adfValue");
}
