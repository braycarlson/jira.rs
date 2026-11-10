#[derive(Clone, Copy, Debug)]
pub struct StatusCreate;

impl StatusCreate {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const STATUS_CATEGORY: crate::Field<Self, crate::Text> =
        crate::Field::new("statusCategory");
}
