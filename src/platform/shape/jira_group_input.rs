#[derive(Clone, Copy, Debug)]
pub struct JiraGroupInput;

impl JiraGroupInput {
    pub const GROUP_NAME: crate::Field<Self, crate::Text> = crate::Field::new("groupName");
}
