#[derive(Clone, Copy, Debug)]
pub struct BulkStartAgentSessionsBatchRequest;

impl BulkStartAgentSessionsBatchRequest {
    pub const CLIENT_MUTATION_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("clientMutationId");

    pub const INVOCATIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::BulkAgentSessionInvocation>>,
    > = crate::Field::new("invocations");
}
