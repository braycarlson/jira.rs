#[derive(Clone, Copy, Debug)]
pub struct RequestNotificationSubscriptionDTO;

impl RequestNotificationSubscriptionDTO {
    pub const SUBSCRIBED: crate::Field<Self, crate::Boolean> = crate::Field::new("subscribed");
}
