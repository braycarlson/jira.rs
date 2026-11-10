#[derive(Clone, Copy, Debug)]
pub struct SingleRedactionRequest;

impl SingleRedactionRequest {
    pub const CONTENT_ITEM: crate::Field<Self, crate::Object<super::ContentItem>> =
        crate::Field::new("contentItem");

    pub const EXTERNAL_ID: crate::Field<Self, crate::Text> = crate::Field::new("externalId");
    pub const REASON: crate::Field<Self, crate::Text> = crate::Field::new("reason");

    pub const REDACTION_POSITION: crate::Field<Self, crate::Object<super::RedactionPosition>> =
        crate::Field::new("redactionPosition");
}
