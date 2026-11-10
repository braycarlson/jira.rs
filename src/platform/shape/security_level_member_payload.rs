#[derive(Clone, Copy, Debug)]
pub struct SecurityLevelMemberPayload;

impl SecurityLevelMemberPayload {
    pub const PARAMETER: crate::Field<Self, crate::Text> = crate::Field::new("parameter");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
