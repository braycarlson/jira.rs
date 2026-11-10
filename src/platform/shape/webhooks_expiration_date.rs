#[derive(Clone, Copy, Debug)]
pub struct WebhooksExpirationDate;

impl WebhooksExpirationDate {
    pub const EXPIRATION_DATE: crate::Field<Self, crate::Any> = crate::Field::new("expirationDate");
}
