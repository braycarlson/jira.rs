#[derive(Clone, Copy, Debug)]
pub struct WorkflowCapabilities;

impl WorkflowCapabilities {
    pub const CONNECT_RULES: crate::Field<
        Self,
        crate::List<crate::Object<super::AvailableWorkflowConnectRule>>,
    > = crate::Field::new("connectRules");

    pub const EDITOR_SCOPE: crate::Field<Self, crate::Text> = crate::Field::new("editorScope");

    pub const FORGE_RULES: crate::Field<
        Self,
        crate::List<crate::Object<super::AvailableWorkflowForgeRule>>,
    > = crate::Field::new("forgeRules");

    pub const PROJECT_TYPES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("projectTypes");

    pub const SYSTEM_RULES: crate::Field<
        Self,
        crate::List<crate::Object<super::AvailableWorkflowSystemRule>>,
    > = crate::Field::new("systemRules");

    pub const TRIGGER_RULES: crate::Field<
        Self,
        crate::List<crate::Object<super::AvailableWorkflowTriggers>>,
    > = crate::Field::new("triggerRules");
}
