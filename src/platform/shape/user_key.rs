#[derive(Clone, Copy, Debug)]
pub struct UserKey;

impl UserKey {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
}
