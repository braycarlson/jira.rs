#[derive(Clone, Copy, Debug)]
pub struct CustomFieldContextDefaultValuesUpdate;

impl CustomFieldContextDefaultValuesUpdate {
    pub const DEFAULT_VALUES: crate::Field<
        Self,
        crate::List<crate::Object<super::IssueTypeDefaultValueUpdate>>,
    > = crate::Field::new("defaultValues");
}
