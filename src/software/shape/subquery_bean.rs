#[derive(Clone, Copy, Debug)]
pub struct SubqueryBean;

impl SubqueryBean {
    pub const QUERY: crate::Field<Self, crate::Text> = crate::Field::new("query");
}
