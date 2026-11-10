#[derive(Clone, Copy, Debug)]
pub struct TargetStatus;

impl TargetStatus {
    pub const STATUSES: crate::Field<Self, crate::Map<crate::List<crate::Text>>> =
        crate::Field::new("statuses");
}
