#[derive(Clone, Copy, Debug)]
pub struct ContextDefaultValues;

impl ContextDefaultValues {
    pub const CONTEXT_ID: crate::Field<Self, crate::Integer> = crate::Field::new("contextId");

    pub const DEFAULT_VALUES: crate::Field<
        Self,
        crate::List<crate::Object<super::IssueTypeDefaultValue>>,
    > = crate::Field::new("defaultValues");
}
