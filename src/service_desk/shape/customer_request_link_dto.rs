#[derive(Clone, Copy, Debug)]
pub struct CustomerRequestLinkDTO;

impl CustomerRequestLinkDTO {
    pub const AGENT: crate::Field<Self, crate::Text> = crate::Field::new("agent");
    pub const JIRA_REST: crate::Field<Self, crate::Text> = crate::Field::new("jiraRest");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
    pub const WEB: crate::Field<Self, crate::Text> = crate::Field::new("web");
}
