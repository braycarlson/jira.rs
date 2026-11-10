#[derive(Clone, Copy, Debug)]
pub struct ProjectCustomTemplateCreateRequestDTO;

impl ProjectCustomTemplateCreateRequestDTO {
    pub const DETAILS: crate::Field<Self, crate::Object<super::CustomTemplatesProjectDetails>> =
        crate::Field::new("details");

    pub const TEMPLATE: crate::Field<Self, crate::Object<super::CustomTemplateRequestDTO>> =
        crate::Field::new("template");
}
