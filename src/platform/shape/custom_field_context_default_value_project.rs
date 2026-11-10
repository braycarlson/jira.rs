#[derive(Clone, Copy, Debug)]
pub struct CustomFieldContextDefaultValueProject;

impl CustomFieldContextDefaultValueProject {
    pub const CONTEXT_ID: crate::Field<Self, crate::Text> = crate::Field::new("contextId");
    pub const PROJECT_ID: crate::Field<Self, crate::Text> = crate::Field::new("projectId");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
