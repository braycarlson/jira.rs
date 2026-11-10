#[derive(Clone, Copy, Debug)]
pub struct MultipleCustomFieldValuesUpdateDetails;

impl MultipleCustomFieldValuesUpdateDetails {
    pub const UPDATES: crate::Field<
        Self,
        crate::List<crate::Object<super::MultipleCustomFieldValuesUpdate>>,
    > = crate::Field::new("updates");
}
