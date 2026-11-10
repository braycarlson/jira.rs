#[derive(Clone, Copy, Debug)]
pub struct CustomFieldConfigurations;

impl CustomFieldConfigurations {
    pub const CONFIGURATIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::ContextualConfiguration>>,
    > = crate::Field::new("configurations");
}
