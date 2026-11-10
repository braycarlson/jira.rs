#[derive(Clone, Copy, Debug)]
pub struct BoardConfigBeanColumnConfig;

impl BoardConfigBeanColumnConfig {
    pub const COLUMNS: crate::Field<
        Self,
        crate::List<crate::Object<super::BoardConfigBeanColumnConfigColumns>>,
    > = crate::Field::new("columns");

    pub const CONSTRAINT_TYPE: crate::Field<Self, crate::Text> =
        crate::Field::new("constraintType");
}
