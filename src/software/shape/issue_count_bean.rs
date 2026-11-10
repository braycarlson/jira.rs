#[derive(Clone, Copy, Debug)]
pub struct IssueCountBean;

impl IssueCountBean {
    pub const COUNT: crate::Field<Self, crate::Integer> = crate::Field::new("count");
}
