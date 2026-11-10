#[derive(Clone, Copy, Debug)]
pub struct StatusProjectUsage;

impl StatusProjectUsage {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
