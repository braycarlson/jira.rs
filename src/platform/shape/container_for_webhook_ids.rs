#[derive(Clone, Copy, Debug)]
pub struct ContainerForWebhookIDs;

impl ContainerForWebhookIDs {
    pub const WEBHOOK_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("webhookIds");
}
