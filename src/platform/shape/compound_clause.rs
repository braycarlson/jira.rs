#[derive(Clone, Copy, Debug)]
pub struct CompoundClause;

impl CompoundClause {
    pub const CLAUSES: crate::Field<Self, crate::List<crate::Any>> = crate::Field::new("clauses");
    pub const OPERATOR: crate::Field<Self, crate::Text> = crate::Field::new("operator");
}
