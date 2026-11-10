#[derive(Clone, Copy, Debug)]
pub struct JQLFunctionPrecomputationUpdateRequestBean;

impl JQLFunctionPrecomputationUpdateRequestBean {
    pub const VALUES: crate::Field<
        Self,
        crate::List<crate::Object<super::JQLFunctionPrecomputationUpdateBean>>,
    > = crate::Field::new("values");
}
