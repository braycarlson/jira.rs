#[derive(Clone, Copy, Debug)]
pub struct SwimlanePayload;

impl SwimlanePayload {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const JQL_QUERY: crate::Field<Self, crate::Text> = crate::Field::new("jqlQuery");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
