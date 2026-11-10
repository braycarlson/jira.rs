#[derive(Clone, Copy, Debug)]
pub struct GetDeploymentByKeyReplyCommands;

impl GetDeploymentByKeyReplyCommands {
    pub const COMMAND: crate::Field<Self, crate::Text> = crate::Field::new("command");
}
