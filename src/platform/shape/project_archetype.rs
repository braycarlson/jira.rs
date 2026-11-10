#[derive(Clone, Copy, Debug)]
pub struct ProjectArchetype;

impl ProjectArchetype {
    pub const REAL_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("realType");
    pub const STYLE: crate::Field<Self, crate::Text> = crate::Field::new("style");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
