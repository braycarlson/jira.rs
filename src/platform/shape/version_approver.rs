#[derive(Clone, Copy, Debug)]
pub struct VersionApprover;

impl VersionApprover {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
    pub const DECLINE_REASON: crate::Field<Self, crate::Text> = crate::Field::new("declineReason");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
}
