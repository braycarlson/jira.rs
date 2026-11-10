#[derive(Clone, Copy, Debug)]
pub struct ServiceDeskDTO;

impl ServiceDeskDTO {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const LINKS: crate::Field<Self, crate::Object<super::SelfLinkDTO>> =
        crate::Field::new("_links");

    pub const PROJECT_ID: crate::Field<Self, crate::Text> = crate::Field::new("projectId");
    pub const PROJECT_KEY: crate::Field<Self, crate::Text> = crate::Field::new("projectKey");
    pub const PROJECT_NAME: crate::Field<Self, crate::Text> = crate::Field::new("projectName");

    pub const PROJECT_TYPE_KEY: crate::Field<Self, crate::Text> =
        crate::Field::new("projectTypeKey");
}
