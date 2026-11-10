#[derive(Clone, Copy, Debug)]
pub struct ServiceDeskCustomerDTO;

impl ServiceDeskCustomerDTO {
    pub const ACCOUNT_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("accountIds");

    pub const USERNAMES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("usernames");
}
