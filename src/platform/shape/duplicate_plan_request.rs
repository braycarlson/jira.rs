#[derive(Clone, Copy, Debug)]
pub struct DuplicatePlanRequest;

impl DuplicatePlanRequest {
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
