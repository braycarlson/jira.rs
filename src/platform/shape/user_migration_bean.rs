#[derive(Clone, Copy, Debug)]
pub struct UserMigrationBean;

impl UserMigrationBean {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const USERNAME: crate::Field<Self, crate::Text> = crate::Field::new("username");
}
