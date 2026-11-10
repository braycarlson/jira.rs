#[derive(Clone, Copy, Debug)]
pub struct ProjectFieldBean;

impl ProjectFieldBean {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");
    pub const IS_REQUIRED: crate::Field<Self, crate::Boolean> = crate::Field::new("isRequired");
    pub const PROJECT_ID: crate::Field<Self, crate::Integer> = crate::Field::new("projectId");
    pub const WORK_TYPE_ID: crate::Field<Self, crate::Integer> = crate::Field::new("workTypeId");
}
