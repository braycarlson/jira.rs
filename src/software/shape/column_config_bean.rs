#[derive(Clone, Copy, Debug)]
pub struct ColumnConfigBean;

impl ColumnConfigBean {
    pub const COLUMNS: crate::Field<
        Self,
        crate::List<crate::Object<super::ColumnConfigBeanColumns>>,
    > = crate::Field::new("columns");

    pub const CONSTRAINT_TYPE: crate::Field<Self, crate::Text> =
        crate::Field::new("constraintType");
}
