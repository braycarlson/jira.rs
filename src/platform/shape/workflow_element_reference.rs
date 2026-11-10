#[derive(Clone, Copy, Debug)]
pub struct WorkflowElementReference;

impl WorkflowElementReference {
    pub const PROPERTY_KEY: crate::Field<Self, crate::Text> = crate::Field::new("propertyKey");
    pub const RULE_ID: crate::Field<Self, crate::Text> = crate::Field::new("ruleId");

    pub const STATUS_MAPPING_REFERENCE: crate::Field<
        Self,
        crate::Object<super::ProjectAndIssueTypePair>,
    > = crate::Field::new("statusMappingReference");

    pub const STATUS_REFERENCE: crate::Field<Self, crate::Text> =
        crate::Field::new("statusReference");

    pub const TRANSITION_ID: crate::Field<Self, crate::Text> = crate::Field::new("transitionId");
}
