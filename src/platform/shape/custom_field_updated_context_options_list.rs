#[derive(Clone, Copy, Debug)]
pub struct CustomFieldUpdatedContextOptionsList;

impl CustomFieldUpdatedContextOptionsList {
    pub const OPTIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::CustomFieldOptionUpdate>>,
    > = crate::Field::new("options");
}
