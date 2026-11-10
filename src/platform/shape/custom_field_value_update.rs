#[derive(Clone, Copy, Debug)]
pub struct CustomFieldValueUpdate;

impl CustomFieldValueUpdate {
    pub const ISSUE_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("issueIds");

    pub const VALUE: crate::Field<Self, crate::Any> = crate::Field::new("value");
}
