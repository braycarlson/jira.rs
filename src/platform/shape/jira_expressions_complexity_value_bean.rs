#[derive(Clone, Copy, Debug)]
pub struct JiraExpressionsComplexityValueBean;

impl JiraExpressionsComplexityValueBean {
    pub const LIMIT: crate::Field<Self, crate::Integer> = crate::Field::new("limit");
    pub const VALUE: crate::Field<Self, crate::Integer> = crate::Field::new("value");
}
