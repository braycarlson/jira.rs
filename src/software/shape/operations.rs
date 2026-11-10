#[derive(Clone, Copy, Debug)]
pub struct Operations;

impl Operations {
    pub const LINK_GROUPS: crate::Field<Self, crate::List<crate::Object<super::LinkGroup>>> =
        crate::Field::new("linkGroups");
}
