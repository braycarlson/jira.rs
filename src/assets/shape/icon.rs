#[derive(Clone, Copy, Debug)]
pub struct Icon;

impl Icon {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const URL16: crate::Field<Self, crate::Text> = crate::Field::new("url16");
    pub const URL48: crate::Field<Self, crate::Text> = crate::Field::new("url48");
}
