#[derive(Clone, Copy, Debug)]
pub struct ConditionGroupPayload;

impl ConditionGroupPayload {
    pub const CONDITIONS: crate::Field<Self, crate::List<crate::Object<super::RulePayload>>> =
        crate::Field::new("conditions");

    pub const CONDITION_GROUP: crate::Field<Self, crate::List<crate::Object<Self>>> =
        crate::Field::new("conditionGroup");

    pub const OPERATION: crate::Field<Self, crate::Text> = crate::Field::new("operation");
}
