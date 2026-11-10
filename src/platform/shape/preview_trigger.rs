#[derive(Clone, Copy, Debug)]
pub struct PreviewTrigger;

impl PreviewTrigger {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const RULE_KEY: crate::Field<Self, crate::Text> = crate::Field::new("ruleKey");
}
