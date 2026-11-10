#[derive(Clone, Copy, Debug)]
pub struct SearchWarningLimitDetails;

impl SearchWarningLimitDetails {
    pub const ACTUAL: crate::Field<Self, crate::Integer> = crate::Field::new("actual");
    pub const ARGUMENTS: crate::Field<Self, crate::Text> = crate::Field::new("arguments");
    pub const CLAUSE: crate::Field<Self, crate::Text> = crate::Field::new("clause");
    pub const LIMIT: crate::Field<Self, crate::Integer> = crate::Field::new("limit");
}
