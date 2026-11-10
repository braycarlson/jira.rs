#[derive(Clone, Copy, Debug)]
pub struct ProjectIdentifiers;

impl ProjectIdentifiers {
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
