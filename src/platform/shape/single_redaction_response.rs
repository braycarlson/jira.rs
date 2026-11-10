#[derive(Clone, Copy, Debug)]
pub struct SingleRedactionResponse;

impl SingleRedactionResponse {
    pub const EXTERNAL_ID: crate::Field<Self, crate::Text> = crate::Field::new("externalId");
    pub const SUCCESSFUL: crate::Field<Self, crate::Boolean> = crate::Field::new("successful");
}
