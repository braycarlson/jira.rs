#[derive(Clone, Copy, Debug)]
pub struct LinkGroupLinks;

impl LinkGroupLinks {
    pub const HREF: crate::Field<Self, crate::Text> = crate::Field::new("href");
    pub const ICON_CLASS: crate::Field<Self, crate::Text> = crate::Field::new("iconClass");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const LABEL: crate::Field<Self, crate::Text> = crate::Field::new("label");
    pub const STYLE_CLASS: crate::Field<Self, crate::Text> = crate::Field::new("styleClass");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");
    pub const WEIGHT: crate::Field<Self, crate::Integer> = crate::Field::new("weight");
}
