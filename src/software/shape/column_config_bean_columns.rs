#[derive(Clone, Copy, Debug)]
pub struct ColumnConfigBeanColumns;

impl ColumnConfigBeanColumns {
    pub const MAX: crate::Field<Self, crate::Integer> = crate::Field::new("max");
    pub const MIN: crate::Field<Self, crate::Integer> = crate::Field::new("min");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const STATUSES: crate::Field<
        Self,
        crate::List<crate::Object<super::ColumnConfigBeanColumnsStatuses>>,
    > = crate::Field::new("statuses");
}
