#[derive(Clone, Copy, Debug)]
pub struct BulkStartUniformAgentSessionsRequest;

impl BulkStartUniformAgentSessionsRequest {
    pub const AGENT_ID: crate::Field<Self, crate::Text> = crate::Field::new("agentId");

    pub const CLIENT_MUTATION_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("clientMutationId");

    pub const ISSUE_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("issueIds");

    pub const USER_PROMPT: crate::Field<Self, crate::Text> = crate::Field::new("userPrompt");
}
