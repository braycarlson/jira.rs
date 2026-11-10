#[derive(Clone, Copy, Debug)]
pub struct ApprovalDTO;

impl ApprovalDTO {
    pub const APPROVERS: crate::Field<Self, crate::List<crate::Object<super::ApproverDTO>>> =
        crate::Field::new("approvers");

    pub const CAN_ANSWER_APPROVAL: crate::Field<Self, crate::Boolean> =
        crate::Field::new("canAnswerApproval");

    pub const COMPLETED_DATE: crate::Field<Self, crate::Object<super::DateDTO>> =
        crate::Field::new("completedDate");

    pub const CREATED_DATE: crate::Field<Self, crate::Object<super::DateDTO>> =
        crate::Field::new("createdDate");

    pub const FINAL_DECISION: crate::Field<Self, crate::Text> = crate::Field::new("finalDecision");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const LINKS: crate::Field<Self, crate::Object<super::SelfLinkDTO>> =
        crate::Field::new("_links");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
