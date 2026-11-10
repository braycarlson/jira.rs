#[derive(Clone, Copy, Debug)]
pub struct Tickets;

impl Tickets {
    pub const ALL_TICKETS_QUERY: crate::Field<Self, crate::Text> =
        crate::Field::new("allTicketsQuery");

    pub const ALL_TICKETS_QUERY_BY_SITE: crate::Field<Self, crate::Any> =
        crate::Field::new("allTicketsQueryBySite");

    pub const TICKETS: crate::Field<Self, crate::List<crate::Object<super::Ticket>>> =
        crate::Field::new("tickets");
}
