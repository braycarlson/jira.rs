#[derive(Clone, Copy, Debug)]
pub struct MultipleCustomFieldValuesUpdate;

impl MultipleCustomFieldValuesUpdate {
    pub const CUSTOM_FIELD: crate::Field<Self, crate::Text> = crate::Field::new("customField");

    pub const ISSUE_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("issueIds");

    pub const VALUE: crate::Field<Self, crate::Any> = crate::Field::new("value");
}
