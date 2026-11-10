#[derive(Clone, Copy, Debug)]
pub struct WebhookRegistrationDetails;

impl WebhookRegistrationDetails {
    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");

    pub const WEBHOOKS: crate::Field<Self, crate::List<crate::Object<super::WebhookDetails>>> =
        crate::Field::new("webhooks");
}
