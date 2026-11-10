#[derive(Clone, Copy, Debug)]
pub struct RequestTypeGroupDTO;

impl RequestTypeGroupDTO {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
