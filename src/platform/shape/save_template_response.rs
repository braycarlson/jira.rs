#[derive(Clone, Copy, Debug)]
pub struct SaveTemplateResponse;

impl SaveTemplateResponse {
    pub const PROJECT_TEMPLATE_KEY: crate::Field<Self, crate::Object<super::ProjectTemplateKey>> =
        crate::Field::new("projectTemplateKey");
}
