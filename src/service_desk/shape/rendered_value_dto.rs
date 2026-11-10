#[derive(Clone, Copy, Debug)]
pub struct RenderedValueDTO;

impl RenderedValueDTO {
    pub const HTML: crate::Field<Self, crate::Text> = crate::Field::new("html");
}
