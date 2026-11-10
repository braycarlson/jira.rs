#[derive(Clone, Copy, Debug)]
pub struct LinkableUserLinkDTO;

impl LinkableUserLinkDTO {
    pub const LINKS: crate::Field<Self, crate::Object<super::UserLinkDTO>> =
        crate::Field::new("_links");
}
