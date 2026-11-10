#[derive(Clone, Copy, Debug)]
pub struct RequestTypeIconLinkDTO;

impl RequestTypeIconLinkDTO {
    pub const ICON_URLS: crate::Field<Self, crate::Map<crate::Text>> =
        crate::Field::new("iconUrls");
}
