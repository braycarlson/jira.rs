#[derive(Clone, Copy, Debug)]
pub struct SaveTemplateRequest;

impl SaveTemplateRequest {
    pub const TEMPLATE_DESCRIPTION: crate::Field<Self, crate::Text> =
        crate::Field::new("templateDescription");

    pub const TEMPLATE_FROM_PROJECT_REQUEST: crate::Field<
        Self,
        crate::Object<super::SaveProjectTemplateRequest>,
    > = crate::Field::new("templateFromProjectRequest");

    pub const TEMPLATE_NAME: crate::Field<Self, crate::Text> = crate::Field::new("templateName");
}
