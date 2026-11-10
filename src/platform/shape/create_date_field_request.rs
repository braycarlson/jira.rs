#[derive(Clone, Copy, Debug)]
pub struct CreateDateFieldRequest;

impl CreateDateFieldRequest {
    pub const DATE_CUSTOM_FIELD_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("dateCustomFieldId");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
