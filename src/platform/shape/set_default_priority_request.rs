#[derive(Clone, Copy, Debug)]
pub struct SetDefaultPriorityRequest;

impl SetDefaultPriorityRequest {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
