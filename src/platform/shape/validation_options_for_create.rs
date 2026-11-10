#[derive(Clone, Copy, Debug)]
pub struct ValidationOptionsForCreate;

impl ValidationOptionsForCreate {
    pub const LEVELS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("levels");
}
