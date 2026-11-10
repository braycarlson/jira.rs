#[derive(Clone, Copy, Debug)]
pub struct CustomFieldContextDefaultValueUpdate;

impl CustomFieldContextDefaultValueUpdate {
    pub const DEFAULT_VALUES: crate::Field<Self, crate::List<crate::Any>> =
        crate::Field::new("defaultValues");
}
