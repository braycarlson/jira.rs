#[derive(Clone, Copy, Debug)]
pub struct ConnectCustomFieldValues;

impl ConnectCustomFieldValues {
    pub const UPDATE_VALUE_LIST: crate::Field<
        Self,
        crate::List<crate::Object<super::ConnectCustomFieldValue>>,
    > = crate::Field::new("updateValueList");
}
