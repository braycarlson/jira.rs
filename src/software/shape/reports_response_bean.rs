#[derive(Clone, Copy, Debug)]
pub struct ReportsResponseBean;

impl ReportsResponseBean {
    pub const REPORTS: crate::Field<Self, crate::List<crate::Any>> = crate::Field::new("reports");
}
