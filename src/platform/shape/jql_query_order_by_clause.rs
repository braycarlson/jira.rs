#[derive(Clone, Copy, Debug)]
pub struct JQLQueryOrderByClause;

impl JQLQueryOrderByClause {
    pub const FIELDS: crate::Field<
        Self,
        crate::List<crate::Object<super::JQLQueryOrderByClauseElement>>,
    > = crate::Field::new("fields");
}
