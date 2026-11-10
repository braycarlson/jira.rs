#[derive(Clone, Copy, Debug)]
pub struct JQLQueriesToParse;

impl JQLQueriesToParse {
    pub const QUERIES: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("queries");
}
