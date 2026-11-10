#[derive(Clone, Copy, Debug)]
pub struct RequestTypeIconDTO;

impl RequestTypeIconDTO {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const LINKS: crate::Field<Self, crate::Object<super::RequestTypeIconLinkDTO>> =
        crate::Field::new("_links");
}
