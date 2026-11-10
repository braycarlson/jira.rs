#[derive(Clone, Copy, Debug)]
pub struct RegisteredWebhook;

impl RegisteredWebhook {
    pub const CREATED_WEBHOOK_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("createdWebhookId");

    pub const ERRORS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("errors");
}
