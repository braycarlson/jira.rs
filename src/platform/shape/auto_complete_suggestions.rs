#[derive(Clone, Copy, Debug)]
pub struct AutoCompleteSuggestions;

impl AutoCompleteSuggestions {
    pub const RESULTS: crate::Field<
        Self,
        crate::List<crate::Object<super::AutoCompleteSuggestion>>,
    > = crate::Field::new("results");
}
