#[derive(Clone, Copy, Debug)]
pub struct ProjectUsage;

impl ProjectUsage {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
