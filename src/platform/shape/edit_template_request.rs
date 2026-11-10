#[derive(Clone, Copy, Debug)]
pub struct EditTemplateRequest;

impl EditTemplateRequest {
    pub const TEMPLATE_DESCRIPTION: crate::Field<Self, crate::Text> =
        crate::Field::new("templateDescription");

    pub const TEMPLATE_GENERATION_OPTIONS: crate::Field<
        Self,
        crate::Object<super::CustomTemplateOptions>,
    > = crate::Field::new("templateGenerationOptions");

    pub const TEMPLATE_KEY: crate::Field<Self, crate::Text> = crate::Field::new("templateKey");
    pub const TEMPLATE_NAME: crate::Field<Self, crate::Text> = crate::Field::new("templateName");
}
