#[derive(Clone, Copy, Debug)]
pub struct StatusUpdate;

impl StatusUpdate {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const STATUS_CATEGORY: crate::Field<Self, crate::Text> =
        crate::Field::new("statusCategory");
}
