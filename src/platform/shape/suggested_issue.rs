#[derive(Clone, Copy, Debug)]
pub struct SuggestedIssue;

impl SuggestedIssue {
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const IMAGE: crate::Field<Self, crate::Text> = crate::Field::new("img");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const KEY_HTML: crate::Field<Self, crate::Text> = crate::Field::new("keyHtml");
    pub const SUMMARY: crate::Field<Self, crate::Text> = crate::Field::new("summary");
    pub const SUMMARY_TEXT: crate::Field<Self, crate::Text> = crate::Field::new("summaryText");
}
