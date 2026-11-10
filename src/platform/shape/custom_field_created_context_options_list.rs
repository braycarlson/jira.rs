#[derive(Clone, Copy, Debug)]
pub struct CustomFieldCreatedContextOptionsList;

impl CustomFieldCreatedContextOptionsList {
    pub const OPTIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::CustomFieldContextOption>>,
    > = crate::Field::new("options");
}
