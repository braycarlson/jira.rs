#[derive(Clone, Copy, Debug)]
pub struct AvailableWorkflowConnectRule;

impl AvailableWorkflowConnectRule {
    pub const ADDON_KEY: crate::Field<Self, crate::Text> = crate::Field::new("addonKey");
    pub const CREATE_URL: crate::Field<Self, crate::Text> = crate::Field::new("createUrl");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const EDIT_URL: crate::Field<Self, crate::Text> = crate::Field::new("editUrl");
    pub const MODULE_KEY: crate::Field<Self, crate::Text> = crate::Field::new("moduleKey");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const RULE_KEY: crate::Field<Self, crate::Text> = crate::Field::new("ruleKey");
    pub const RULE_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("ruleType");
    pub const VIEW_URL: crate::Field<Self, crate::Text> = crate::Field::new("viewUrl");
}
