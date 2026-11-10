#[derive(Clone, Copy, Debug)]
pub struct GetRemoteLinkByIDReplyStatus;

impl GetRemoteLinkByIDReplyStatus {
    pub const APPEARANCE: crate::Field<Self, crate::Text> = crate::Field::new("appearance");
    pub const LABEL: crate::Field<Self, crate::Text> = crate::Field::new("label");
}
