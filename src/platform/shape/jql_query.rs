#[derive(Clone, Copy, Debug)]
pub struct JQLQuery;

impl JQLQuery {
    pub const ORDER_BY: crate::Field<Self, crate::Object<super::JQLQueryOrderByClause>> =
        crate::Field::new("orderBy");

    pub const WHERE: crate::Field<Self, crate::Any> = crate::Field::new("where");
}
