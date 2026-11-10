#[derive(Clone, Copy, Debug)]
pub struct GetDeploymentByKeyReplyEnvironment;

impl GetDeploymentByKeyReplyEnvironment {
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
