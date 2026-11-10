#[derive(Clone, Copy, Debug)]
pub struct IssuePickerSuggestions;

impl IssuePickerSuggestions {
    pub const SECTIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::IssuePickerSuggestionsIssueType>>,
    > = crate::Field::new("sections");
}
