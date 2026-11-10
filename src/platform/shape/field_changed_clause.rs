#[derive(Clone, Copy, Debug)]
pub struct FieldChangedClause;

impl FieldChangedClause {
    pub const FIELD: crate::Field<Self, crate::Object<super::JQLQueryField>> =
        crate::Field::new("field");

    pub const OPERATOR: crate::Field<Self, crate::Text> = crate::Field::new("operator");

    pub const PREDICATES: crate::Field<
        Self,
        crate::List<crate::Object<super::JQLQueryClauseTimePredicate>>,
    > = crate::Field::new("predicates");
}
