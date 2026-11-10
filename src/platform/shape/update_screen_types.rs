#[derive(Clone, Copy, Debug)]
pub struct UpdateScreenTypes;

impl UpdateScreenTypes {
    pub const CREATE: crate::Field<Self, crate::Text> = crate::Field::new("create");
    pub const DEFAULT: crate::Field<Self, crate::Text> = crate::Field::new("default");
    pub const EDIT: crate::Field<Self, crate::Text> = crate::Field::new("edit");
    pub const VIEW: crate::Field<Self, crate::Text> = crate::Field::new("view");
}
