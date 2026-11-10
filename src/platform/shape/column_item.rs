#[derive(Clone, Copy, Debug)]
pub struct ColumnItem;

impl ColumnItem {
    pub const LABEL: crate::Field<Self, crate::Text> = crate::Field::new("label");
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
