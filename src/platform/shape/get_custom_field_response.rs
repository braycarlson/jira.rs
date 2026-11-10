#[derive(Clone, Copy, Debug)]
pub struct GetCustomFieldResponse;

impl GetCustomFieldResponse {
    pub const CUSTOM_FIELD_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("customFieldId");

    pub const FILTER: crate::Field<Self, crate::Boolean> = crate::Field::new("filter");
}
