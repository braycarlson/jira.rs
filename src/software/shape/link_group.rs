#[derive(Clone, Copy, Debug)]
pub struct LinkGroup;

impl LinkGroup {
    pub const GROUPS: crate::Field<Self, crate::List<crate::Object<Self>>> =
        crate::Field::new("groups");

    pub const HEADER: crate::Field<Self, crate::Object<super::LinkGroupHeader>> =
        crate::Field::new("header");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const LINKS: crate::Field<Self, crate::List<crate::Object<super::LinkGroupLinks>>> =
        crate::Field::new("links");

    pub const STYLE_CLASS: crate::Field<Self, crate::Text> = crate::Field::new("styleClass");
    pub const WEIGHT: crate::Field<Self, crate::Integer> = crate::Field::new("weight");
}
