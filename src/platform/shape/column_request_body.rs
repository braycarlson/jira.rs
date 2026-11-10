#[derive(Clone, Copy, Debug)]
pub struct ColumnRequestBody;

impl ColumnRequestBody {
    pub const COLUMNS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("columns");
}
