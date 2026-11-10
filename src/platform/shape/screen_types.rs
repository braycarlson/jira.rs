#[derive(Clone, Copy, Debug)]
pub struct ScreenTypes;

impl ScreenTypes {
    pub const CREATE: crate::Field<Self, crate::Integer> = crate::Field::new("create");
    pub const DEFAULT: crate::Field<Self, crate::Integer> = crate::Field::new("default");
    pub const EDIT: crate::Field<Self, crate::Integer> = crate::Field::new("edit");
    pub const VIEW: crate::Field<Self, crate::Integer> = crate::Field::new("view");
}
