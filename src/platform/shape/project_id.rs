#[derive(Clone, Copy, Debug)]
pub struct ProjectID;

impl ProjectID {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
