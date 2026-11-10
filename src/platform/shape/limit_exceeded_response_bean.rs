#[derive(Clone, Copy, Debug)]
pub struct LimitExceededResponseBean;

impl LimitExceededResponseBean {
    pub const ALLOWED_LIMIT_MAX: crate::Field<Self, crate::Integer> =
        crate::Field::new("max_allowed_limit");

    pub const COUNT_CURRENT: crate::Field<Self, crate::Integer> =
        crate::Field::new("current_count");

    pub const ENTITY_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("entity_type");
    pub const ERROR_CODE: crate::Field<Self, crate::Text> = crate::Field::new("error_code");
    pub const LIMIT_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("limit_type");
    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("message");
    pub const SCOPE_ID: crate::Field<Self, crate::Text> = crate::Field::new("scope_id");
}
