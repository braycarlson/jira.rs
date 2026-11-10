#[derive(Clone, Copy, Debug)]
pub struct SetDefaultResolutionRequest;

impl SetDefaultResolutionRequest {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
