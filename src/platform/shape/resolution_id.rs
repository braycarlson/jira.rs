#[derive(Clone, Copy, Debug)]
pub struct ResolutionID;

impl ResolutionID {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
