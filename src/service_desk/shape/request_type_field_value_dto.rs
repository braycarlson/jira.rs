#[derive(Clone, Copy, Debug)]
pub struct RequestTypeFieldValueDTO;

impl RequestTypeFieldValueDTO {
    pub const CHILDREN: crate::Field<Self, crate::List<crate::Object<Self>>> =
        crate::Field::new("children");

    pub const LABEL: crate::Field<Self, crate::Text> = crate::Field::new("label");
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
