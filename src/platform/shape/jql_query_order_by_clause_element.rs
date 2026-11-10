#[derive(Clone, Copy, Debug)]
pub struct JQLQueryOrderByClauseElement;

impl JQLQueryOrderByClauseElement {
    pub const DIRECTION: crate::Field<Self, crate::Text> = crate::Field::new("direction");

    pub const FIELD: crate::Field<Self, crate::Object<super::JQLQueryField>> =
        crate::Field::new("field");
}
