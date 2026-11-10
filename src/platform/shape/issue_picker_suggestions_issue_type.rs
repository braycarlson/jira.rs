#[derive(Clone, Copy, Debug)]
pub struct IssuePickerSuggestionsIssueType;

impl IssuePickerSuggestionsIssueType {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const ISSUES: crate::Field<Self, crate::List<crate::Object<super::SuggestedIssue>>> =
        crate::Field::new("issues");

    pub const LABEL: crate::Field<Self, crate::Text> = crate::Field::new("label");
    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("msg");
    pub const SUB: crate::Field<Self, crate::Text> = crate::Field::new("sub");
}
