#[derive(Clone, Copy, Debug)]
pub struct QueueDTO;

impl QueueDTO {
    pub const FIELDS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("fields");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const ISSUE_COUNT: crate::Field<Self, crate::Integer> = crate::Field::new("issueCount");
    pub const JQL: crate::Field<Self, crate::Text> = crate::Field::new("jql");

    pub const LINKS: crate::Field<Self, crate::Object<super::SelfLinkDTO>> =
        crate::Field::new("_links");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
