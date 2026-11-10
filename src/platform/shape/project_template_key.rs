#[derive(Clone, Copy, Debug)]
pub struct ProjectTemplateKey;

impl ProjectTemplateKey {
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const UUID: crate::Field<Self, crate::Text> = crate::Field::new("uuid");
}
