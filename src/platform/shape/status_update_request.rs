#[derive(Clone, Copy, Debug)]
pub struct StatusUpdateRequest;

impl StatusUpdateRequest {
    pub const STATUSES: crate::Field<Self, crate::List<crate::Object<super::StatusUpdate>>> =
        crate::Field::new("statuses");
}
