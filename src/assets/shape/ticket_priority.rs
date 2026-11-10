#[derive(Clone, Copy, Debug)]
pub struct TicketPriority;

impl TicketPriority {
    pub const ICON_URL: crate::Field<Self, crate::Text> = crate::Field::new("iconUrl");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
