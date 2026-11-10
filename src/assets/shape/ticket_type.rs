#[derive(Clone, Copy, Debug)]
pub struct TicketType;

impl TicketType {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ICON_URL: crate::Field<Self, crate::Text> = crate::Field::new("iconUrl");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
