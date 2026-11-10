#[derive(Clone, Copy, Debug)]
pub struct LinkableCustomerRequestLinkDTO;

impl LinkableCustomerRequestLinkDTO {
    pub const LINKS: crate::Field<Self, crate::Object<super::CustomerRequestLinkDTO>> =
        crate::Field::new("_links");
}
