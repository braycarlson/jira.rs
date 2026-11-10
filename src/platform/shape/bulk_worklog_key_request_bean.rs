#[derive(Clone, Copy, Debug)]
pub struct BulkWorklogKeyRequestBean;

impl BulkWorklogKeyRequestBean {
    pub const REQUESTS: crate::Field<Self, crate::List<crate::Object<super::WorklogCompositeKey>>> =
        crate::Field::new("requests");
}
