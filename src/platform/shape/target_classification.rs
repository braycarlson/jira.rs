#[derive(Clone, Copy, Debug)]
pub struct TargetClassification;

impl TargetClassification {
    pub const CLASSIFICATIONS: crate::Field<Self, crate::Map<crate::List<crate::Text>>> =
        crate::Field::new("classifications");

    pub const ISSUE_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("issueType");

    pub const PROJECT_KEY_OR_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("projectKeyOrId");
}
