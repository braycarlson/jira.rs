#[derive(Clone, Copy, Debug)]
pub struct SaveProjectTemplateRequest;

impl SaveProjectTemplateRequest {
    pub const PROJECT_ID: crate::Field<Self, crate::Integer> = crate::Field::new("projectId");

    pub const TEMPLATE_GENERATION_OPTIONS: crate::Field<
        Self,
        crate::Object<super::CustomTemplateOptions>,
    > = crate::Field::new("templateGenerationOptions");

    pub const TEMPLATE_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("templateType");
}
