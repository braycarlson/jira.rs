#[derive(Clone, Copy, Debug)]
pub struct CustomFieldValueUpdateDetails;

impl CustomFieldValueUpdateDetails {
    pub const UPDATES: crate::Field<
        Self,
        crate::List<crate::Object<super::CustomFieldValueUpdate>>,
    > = crate::Field::new("updates");
}
