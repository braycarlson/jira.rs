#[derive(Clone, Copy, Debug)]
pub struct ValidationOptionsForUpdate;

impl ValidationOptionsForUpdate {
    pub const LEVELS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("levels");
}
