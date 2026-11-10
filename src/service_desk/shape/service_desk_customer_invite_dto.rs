#[derive(Clone, Copy, Debug)]
pub struct ServiceDeskCustomerInviteDTO;

impl ServiceDeskCustomerInviteDTO {
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const EMAIL: crate::Field<Self, crate::Text> = crate::Field::new("email");
}
