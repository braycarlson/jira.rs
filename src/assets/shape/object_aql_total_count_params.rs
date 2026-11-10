#[derive(Clone, Copy, Debug)]
pub struct ObjectAQLTotalCountParams;

impl ObjectAQLTotalCountParams {
    pub const QL_QUERY: crate::Field<Self, crate::Text> = crate::Field::new("qlQuery");
}
