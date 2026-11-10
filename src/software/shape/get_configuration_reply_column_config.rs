#[derive(Clone, Copy, Debug)]
pub struct GetConfigurationReplyColumnConfig;

impl GetConfigurationReplyColumnConfig {
    pub const COLUMNS: crate::Field<
        Self,
        crate::List<crate::Object<super::GetConfigurationReplyColumnConfigColumns>>,
    > = crate::Field::new("columns");

    pub const CONSTRAINT_TYPE: crate::Field<Self, crate::Text> =
        crate::Field::new("constraintType");
}
