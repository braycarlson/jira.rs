#[derive(Clone, Copy, Debug)]
pub struct ScreenSchemeID;

impl ScreenSchemeID {
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
}
