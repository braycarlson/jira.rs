#[derive(Clone, Copy, Debug)]
pub struct SecuritySchemeID;

impl SecuritySchemeID {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
