#[derive(Clone, Copy, Debug)]
pub struct ContainerForRegisteredWebhooks;

impl ContainerForRegisteredWebhooks {
    pub const WEBHOOK_REGISTRATION_RESULT: crate::Field<
        Self,
        crate::List<crate::Object<super::RegisteredWebhook>>,
    > = crate::Field::new("webhookRegistrationResult");
}
