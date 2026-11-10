#[derive(Clone, Copy, Debug)]
pub struct AutoCompleteSuggestion;

impl AutoCompleteSuggestion {
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
