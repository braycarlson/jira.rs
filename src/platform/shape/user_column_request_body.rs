#[derive(Clone, Copy, Debug)]
pub struct UserColumnRequestBody;

impl UserColumnRequestBody {
    pub const COLUMNS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("columns");
}
