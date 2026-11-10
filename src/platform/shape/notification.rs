#[derive(Clone, Copy, Debug)]
pub struct Notification;

impl Notification {
    pub const HTML_BODY: crate::Field<Self, crate::Text> = crate::Field::new("htmlBody");

    pub const RESTRICT: crate::Field<
        Self,
        crate::Object<super::NotificationRecipientsRestrictions>,
    > = crate::Field::new("restrict");

    pub const SUBJECT: crate::Field<Self, crate::Text> = crate::Field::new("subject");
    pub const TEXT_BODY: crate::Field<Self, crate::Text> = crate::Field::new("textBody");

    pub const TO_VALUE: crate::Field<Self, crate::Object<super::NotificationRecipients>> =
        crate::Field::new("to");
}
