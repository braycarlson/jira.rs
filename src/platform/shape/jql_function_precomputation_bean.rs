#[derive(Clone, Copy, Debug)]
pub struct JQLFunctionPrecomputationBean;

impl JQLFunctionPrecomputationBean {
    pub const ARGUMENTS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("arguments");

    pub const CREATED: crate::Field<Self, crate::Text> = crate::Field::new("created");
    pub const ERROR: crate::Field<Self, crate::Text> = crate::Field::new("error");
    pub const FIELD: crate::Field<Self, crate::Text> = crate::Field::new("field");
    pub const FUNCTION_KEY: crate::Field<Self, crate::Text> = crate::Field::new("functionKey");
    pub const FUNCTION_NAME: crate::Field<Self, crate::Text> = crate::Field::new("functionName");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const OPERATOR: crate::Field<Self, crate::Text> = crate::Field::new("operator");
    pub const UPDATED: crate::Field<Self, crate::Text> = crate::Field::new("updated");
    pub const USED: crate::Field<Self, crate::Text> = crate::Field::new("used");
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
