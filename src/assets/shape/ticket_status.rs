#[derive(Clone, Copy, Debug)]
pub struct TicketStatus;

impl TicketStatus {
    pub const COLOR_NAME: crate::Field<Self, crate::Text> = crate::Field::new("colorName");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
