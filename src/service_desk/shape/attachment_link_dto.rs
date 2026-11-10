#[derive(Clone, Copy, Debug)]
pub struct AttachmentLinkDTO;

impl AttachmentLinkDTO {
    pub const CONTENT: crate::Field<Self, crate::Text> = crate::Field::new("content");
    pub const JIRA_REST: crate::Field<Self, crate::Text> = crate::Field::new("jiraRest");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
    pub const THUMBNAIL: crate::Field<Self, crate::Text> = crate::Field::new("thumbnail");
}
