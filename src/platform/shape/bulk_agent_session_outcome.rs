#[derive(Clone, Copy, Debug)]
pub struct BulkAgentSessionOutcome;

impl BulkAgentSessionOutcome {
    pub const AGENT_ID: crate::Field<Self, crate::Text> = crate::Field::new("agentId");

    pub const CONVERSATION_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("conversationId");

    pub const ERROR: crate::Field<Self, crate::Object<super::BulkAgentSessionItemError>> =
        crate::Field::new("error");

    pub const ISSUE_ID: crate::Field<Self, crate::Text> = crate::Field::new("issueId");
    pub const REQUEST_ID: crate::Field<Self, crate::Text> = crate::Field::new("requestId");
    pub const RESULT: crate::Field<Self, crate::Text> = crate::Field::new("result");
}
