#[derive(Clone, Copy, Debug)]
pub struct NotificationSchemeEventTypeID;

impl NotificationSchemeEventTypeID {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
