#[derive(Clone, Copy, Debug)]
pub struct OrganizationDTO;

impl OrganizationDTO {
    pub const CREATED: crate::Field<Self, crate::Object<super::DateDTO>> =
        crate::Field::new("created");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const LINKS: crate::Field<Self, crate::Object<super::SelfLinkDTO>> =
        crate::Field::new("_links");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const SCIM_MANAGED: crate::Field<Self, crate::Boolean> = crate::Field::new("scimManaged");
    pub const UUID: crate::Field<Self, crate::Text> = crate::Field::new("uuid");
}
