#[derive(Clone, Copy, Debug)]
pub struct CustomerRequestActionDTO;

impl CustomerRequestActionDTO {
    pub const ALLOWED: crate::Field<Self, crate::Boolean> = crate::Field::new("allowed");
}
