#[derive(Clone, Copy, Debug)]
pub struct BoardConfigBeanSubQuery;

impl BoardConfigBeanSubQuery {
    pub const QUERY: crate::Field<Self, crate::Text> = crate::Field::new("query");
}
