#[derive(Clone, Copy, Debug)]
pub struct BulkEditShareableEntityResponse;

impl BulkEditShareableEntityResponse {
    pub const ACTION: crate::Field<Self, crate::Text> = crate::Field::new("action");

    pub const ENTITY_ERRORS: crate::Field<
        Self,
        crate::Map<crate::Object<super::BulkEditActionError>>,
    > = crate::Field::new("entityErrors");
}
