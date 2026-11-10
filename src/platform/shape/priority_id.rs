#[derive(Clone, Copy, Debug)]
pub struct PriorityID;

impl PriorityID {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
