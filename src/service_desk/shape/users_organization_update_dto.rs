#[derive(Clone, Copy, Debug)]
pub struct UsersOrganizationUpdateDTO;

impl UsersOrganizationUpdateDTO {
    pub const ACCOUNT_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("accountIds");

    pub const ORGANIZATION_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("organizationId");

    pub const USERNAMES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("usernames");
}
