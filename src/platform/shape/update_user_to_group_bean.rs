#[derive(Clone, Copy, Debug)]
pub struct UpdateUserToGroupBean;

impl UpdateUserToGroupBean {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
