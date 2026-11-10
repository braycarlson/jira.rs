#[derive(Clone, Copy, Debug)]
pub struct OrganizationCreateDTO;

impl OrganizationCreateDTO {
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
