#[derive(Clone, Copy, Debug)]
pub struct Ticket;

impl Ticket {
    pub const CREATED: crate::Field<Self, crate::Text> = crate::Field::new("created");
    pub const GLOBAL_ID: crate::Field<Self, crate::Text> = crate::Field::new("globalId");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");

    pub const PRIORITY: crate::Field<Self, crate::Object<super::TicketPriority>> =
        crate::Field::new("priority");

    pub const REPORTER: crate::Field<Self, crate::Text> = crate::Field::new("reporter");

    pub const STATUS: crate::Field<Self, crate::Object<super::TicketStatus>> =
        crate::Field::new("status");

    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");

    pub const TYPE: crate::Field<Self, crate::Object<super::TicketType>> =
        crate::Field::new("type");

    pub const UPDATED: crate::Field<Self, crate::Text> = crate::Field::new("updated");
    pub const WORKSPACE_ID: crate::Field<Self, crate::Text> = crate::Field::new("workspaceId");
}
