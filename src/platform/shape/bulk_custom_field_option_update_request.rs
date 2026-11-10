#[derive(Clone, Copy, Debug)]
pub struct BulkCustomFieldOptionUpdateRequest;

impl BulkCustomFieldOptionUpdateRequest {
    pub const OPTIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::CustomFieldOptionUpdate>>,
    > = crate::Field::new("options");
}
