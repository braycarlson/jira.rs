#[derive(Clone, Copy, Debug)]
pub struct Status;

impl Status {
    pub const GLOBAL_ID: crate::Field<Self, crate::Text> = crate::Field::new("globalId");
    pub const ICON: crate::Field<Self, crate::Object<super::Icon>> = crate::Field::new("icon");
    pub const RESOLVED: crate::Field<Self, crate::Boolean> = crate::Field::new("resolved");
    pub const WORKSPACE_ID: crate::Field<Self, crate::Text> = crate::Field::new("workspaceId");
}
