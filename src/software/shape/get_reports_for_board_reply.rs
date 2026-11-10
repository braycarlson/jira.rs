#[derive(Clone, Copy, Debug)]
pub struct GetReportsForBoardReply;

impl GetReportsForBoardReply {
    pub const REPORTS: crate::Field<Self, crate::List<crate::Any>> = crate::Field::new("reports");
}
