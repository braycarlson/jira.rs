#[derive(Clone, Copy, Debug)]
pub struct SearchResultWorkTypeParameters;

impl SearchResultWorkTypeParameters {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const IS_REQUIRED: crate::Field<Self, crate::Boolean> = crate::Field::new("isRequired");
    pub const RENDERER_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("rendererType");
    pub const WORK_TYPE_ID: crate::Field<Self, crate::Text> = crate::Field::new("workTypeId");
}
