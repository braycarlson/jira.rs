#[derive(Clone, Copy, Debug)]
pub struct JQLFunctionPrecomputationUpdateBean;

impl JQLFunctionPrecomputationUpdateBean {
    pub const ERROR: crate::Field<Self, crate::Text> = crate::Field::new("error");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
