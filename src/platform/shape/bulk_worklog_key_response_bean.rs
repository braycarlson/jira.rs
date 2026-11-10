#[derive(Clone, Copy, Debug)]
pub struct BulkWorklogKeyResponseBean;

impl BulkWorklogKeyResponseBean {
    pub const WORKLOGS: crate::Field<Self, crate::List<crate::Object<super::WorklogKeyResult>>> =
        crate::Field::new("worklogs");
}
