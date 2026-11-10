#[derive(Clone, Copy, Debug)]
pub struct UpdateNotificationSchemeDetails;

impl UpdateNotificationSchemeDetails {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
