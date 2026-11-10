#[derive(Clone, Copy, Debug)]
pub struct PropertyKey;

impl PropertyKey {
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
