#[derive(Clone, Copy, Debug)]
pub struct ObjectLinks;

impl ObjectLinks {
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
