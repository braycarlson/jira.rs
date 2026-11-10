#[derive(Clone, Copy, Debug)]
pub struct ApprovalDecisionRequestDTO;

impl ApprovalDecisionRequestDTO {
    pub const DECISION: crate::Field<Self, crate::Text> = crate::Field::new("decision");
}
