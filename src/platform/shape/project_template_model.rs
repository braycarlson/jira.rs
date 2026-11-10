#[derive(Clone, Copy, Debug)]
pub struct ProjectTemplateModel;

impl ProjectTemplateModel {
    pub const ARCHETYPE: crate::Field<Self, crate::Object<super::ProjectArchetype>> =
        crate::Field::new("archetype");

    pub const DEFAULT_BOARD_VIEW: crate::Field<Self, crate::Text> =
        crate::Field::new("defaultBoardView");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const LIVE_TEMPLATE_PROJECT_ID_REFERENCE: crate::Field<Self, crate::Integer> =
        crate::Field::new("liveTemplateProjectIdReference");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const PROJECT_TEMPLATE_KEY: crate::Field<Self, crate::Object<super::ProjectTemplateKey>> =
        crate::Field::new("projectTemplateKey");

    pub const SNAPSHOT_TEMPLATE: crate::Field<Self, crate::Map<crate::Any>> =
        crate::Field::new("snapshotTemplate");

    pub const TEMPLATE_GENERATION_OPTIONS: crate::Field<
        Self,
        crate::Object<super::CustomTemplateOptions>,
    > = crate::Field::new("templateGenerationOptions");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
