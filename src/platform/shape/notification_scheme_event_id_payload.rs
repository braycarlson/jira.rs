#[derive(Clone, Copy, Debug)]
pub struct NotificationSchemeEventIDPayload;

impl NotificationSchemeEventIDPayload {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
