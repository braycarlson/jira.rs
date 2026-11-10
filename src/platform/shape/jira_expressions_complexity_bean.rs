#[derive(Clone, Copy, Debug)]
pub struct JiraExpressionsComplexityBean;

impl JiraExpressionsComplexityBean {
    pub const BEANS: crate::Field<Self, crate::Object<super::JiraExpressionsComplexityValueBean>> =
        crate::Field::new("beans");

    pub const EXPENSIVE_OPERATIONS: crate::Field<
        Self,
        crate::Object<super::JiraExpressionsComplexityValueBean>,
    > = crate::Field::new("expensiveOperations");

    pub const PRIMITIVE_VALUES: crate::Field<
        Self,
        crate::Object<super::JiraExpressionsComplexityValueBean>,
    > = crate::Field::new("primitiveValues");

    pub const STEPS: crate::Field<Self, crate::Object<super::JiraExpressionsComplexityValueBean>> =
        crate::Field::new("steps");
}
