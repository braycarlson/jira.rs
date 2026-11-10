#[derive(Clone, Copy, Debug)]
pub struct BulkCustomFieldOptionCreateRequest;

impl BulkCustomFieldOptionCreateRequest {
    pub const OPTIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::CustomFieldOptionCreate>>,
    > = crate::Field::new("options");
}
