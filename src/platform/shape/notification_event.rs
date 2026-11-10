#[derive(Clone, Copy, Debug)]
pub struct NotificationEvent;

impl NotificationEvent {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const TEMPLATE_EVENT: crate::Field<Self, crate::Object<Self>> =
        crate::Field::new("templateEvent");
}
