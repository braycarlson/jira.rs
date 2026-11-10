#[derive(Clone, Copy, Debug)]
pub struct CustomerRequestStatusDTO;

impl CustomerRequestStatusDTO {
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");

    pub const STATUS_CATEGORY: crate::Field<Self, crate::Text> =
        crate::Field::new("statusCategory");

    pub const STATUS_DATE: crate::Field<Self, crate::Object<super::DateDTO>> =
        crate::Field::new("statusDate");
}
