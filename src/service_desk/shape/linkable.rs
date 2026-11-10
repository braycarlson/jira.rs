#[derive(Clone, Copy, Debug)]
pub struct Linkable;

impl Linkable {
    pub const LINKS: crate::Field<Self, crate::Object<super::SelfLinkDTO>> =
        crate::Field::new("_links");
}
