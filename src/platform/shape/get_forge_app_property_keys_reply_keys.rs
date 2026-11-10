#[derive(Clone, Copy, Debug)]
pub struct GetForgeAppPropertyKeysReplyKeys;

impl GetForgeAppPropertyKeysReplyKeys {
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
