#[derive(Clone, Copy, Debug)]
pub struct WorklogIDsRequestBean;

impl WorklogIDsRequestBean {
    pub const IDS: crate::Field<Self, crate::List<crate::Integer>> = crate::Field::new("ids");
}
