#[derive(Clone, Copy, Debug)]
pub struct ProjectRoleUser;

impl ProjectRoleUser {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
}
