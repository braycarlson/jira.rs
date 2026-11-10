#[derive(Clone, Copy, Debug)]
pub struct ApprovalConfiguration;

impl ApprovalConfiguration {
    pub const ACTIVE: crate::Field<Self, crate::Text> = crate::Field::new("active");
    pub const CONDITION_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("conditionType");

    pub const CONDITION_VALUE: crate::Field<Self, crate::Text> =
        crate::Field::new("conditionValue");

    pub const EXCLUDE: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("exclude");
    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");

    pub const PRE_POPULATED_FIELD_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("prePopulatedFieldId");

    pub const TRANSITION_APPROVED: crate::Field<Self, crate::Text> =
        crate::Field::new("transitionApproved");

    pub const TRANSITION_REJECTED: crate::Field<Self, crate::Text> =
        crate::Field::new("transitionRejected");
}
