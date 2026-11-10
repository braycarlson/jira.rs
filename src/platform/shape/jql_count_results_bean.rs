#[derive(Clone, Copy, Debug)]
pub struct JQLCountResultsBean;

impl JQLCountResultsBean {
    pub const COUNT: crate::Field<Self, crate::Integer> = crate::Field::new("count");
}
