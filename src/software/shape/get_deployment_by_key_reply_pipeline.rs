#[derive(Clone, Copy, Debug)]
pub struct GetDeploymentByKeyReplyPipeline;

impl GetDeploymentByKeyReplyPipeline {
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
}
