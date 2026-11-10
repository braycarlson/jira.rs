#[derive(Clone, Copy, Debug)]
pub struct ObjectAQLTotalCountResult;

impl ObjectAQLTotalCountResult {
    pub const COUNT_TOTAL: crate::Field<Self, crate::Integer> = crate::Field::new("totalCount");
}
