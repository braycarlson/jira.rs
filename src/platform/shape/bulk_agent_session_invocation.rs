#[derive(Clone, Copy, Debug)]
pub struct BulkAgentSessionInvocation;

impl BulkAgentSessionInvocation {
    pub const AGENT_ID: crate::Field<Self, crate::Text> = crate::Field::new("agentId");
    pub const ISSUE_ID: crate::Field<Self, crate::Text> = crate::Field::new("issueId");
    pub const REQUEST_ID: crate::Field<Self, crate::Text> = crate::Field::new("requestId");
    pub const USER_PROMPT: crate::Field<Self, crate::Text> = crate::Field::new("userPrompt");
}
