#[derive(Clone, Copy, Debug)]
pub struct SearchWarning;

impl SearchWarning {
    pub const DETAILS: crate::Field<Self, crate::Object<super::SearchWarningLimitDetails>> =
        crate::Field::new("details");

    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("message");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
