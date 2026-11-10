#[derive(Clone, Copy, Debug)]
pub struct DurationDTO;

impl DurationDTO {
    pub const FRIENDLY: crate::Field<Self, crate::Text> = crate::Field::new("friendly");
    pub const MILLIS: crate::Field<Self, crate::Integer> = crate::Field::new("millis");
}
