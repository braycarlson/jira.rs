#[derive(Clone, Copy, Debug)]
pub struct ContentDTO;

impl ContentDTO {
    pub const IFRAME_SOURCE: crate::Field<Self, crate::Text> = crate::Field::new("iframeSrc");
}
