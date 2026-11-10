#[derive(Clone, Copy, Debug)]
pub struct Icon;

impl Icon {
    pub const LINK: crate::Field<Self, crate::Text> = crate::Field::new("link");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");
    pub const URL16X16: crate::Field<Self, crate::Text> = crate::Field::new("url16x16");
}
