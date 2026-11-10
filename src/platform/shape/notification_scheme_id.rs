#[derive(Clone, Copy, Debug)]
pub struct NotificationSchemeID;

impl NotificationSchemeID {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
