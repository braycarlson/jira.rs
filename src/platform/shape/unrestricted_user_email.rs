#[derive(Clone, Copy, Debug)]
pub struct UnrestrictedUserEmail;

impl UnrestrictedUserEmail {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
    pub const EMAIL: crate::Field<Self, crate::Text> = crate::Field::new("email");
}
