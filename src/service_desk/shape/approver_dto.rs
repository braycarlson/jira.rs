#[derive(Clone, Copy, Debug)]
pub struct ApproverDTO;

impl ApproverDTO {
    pub const APPROVER: crate::Field<Self, crate::Object<super::UserDTO>> =
        crate::Field::new("approver");

    pub const APPROVER_DECISION: crate::Field<Self, crate::Text> =
        crate::Field::new("approverDecision");
}
