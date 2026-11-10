#[derive(Clone, Copy, Debug)]
pub struct RuleConfiguration;

impl RuleConfiguration {
    pub const DISABLED: crate::Field<Self, crate::Boolean> = crate::Field::new("disabled");
    pub const TAG: crate::Field<Self, crate::Text> = crate::Field::new("tag");
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
