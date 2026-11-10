#[derive(Clone, Copy, Debug)]
pub struct WorkTypeParameters;

impl WorkTypeParameters {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const IS_REQUIRED: crate::Field<Self, crate::Boolean> = crate::Field::new("isRequired");
    pub const RENDERER_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("rendererType");
    pub const WORK_TYPE_ID: crate::Field<Self, crate::Integer> = crate::Field::new("workTypeId");
}
