#[derive(Clone, Copy, Debug)]
pub struct CustomFieldContextProjectMapping;

impl CustomFieldContextProjectMapping {
    pub const CONTEXT_ID: crate::Field<Self, crate::Text> = crate::Field::new("contextId");

    pub const IS_GLOBAL_CONTEXT: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isGlobalContext");

    pub const PROJECT_ID: crate::Field<Self, crate::Text> = crate::Field::new("projectId");
}
