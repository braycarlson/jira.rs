#[derive(Clone, Copy, Debug)]
pub struct ProjectEmailAddress;

impl ProjectEmailAddress {
    pub const EMAIL_ADDRESS: crate::Field<Self, crate::Text> = crate::Field::new("emailAddress");

    pub const EMAIL_ADDRESS_STATUS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("emailAddressStatus");
}
