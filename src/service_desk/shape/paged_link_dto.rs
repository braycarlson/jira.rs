#[derive(Clone, Copy, Debug)]
pub struct PagedLinkDTO;

impl PagedLinkDTO {
    pub const BASE: crate::Field<Self, crate::Text> = crate::Field::new("base");
    pub const CONTEXT: crate::Field<Self, crate::Text> = crate::Field::new("context");
    pub const NEXT: crate::Field<Self, crate::Text> = crate::Field::new("next");
    pub const PREVIOUS: crate::Field<Self, crate::Text> = crate::Field::new("prev");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
