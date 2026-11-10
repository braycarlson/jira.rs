#[derive(Clone, Copy, Debug)]
pub struct ObjectAQLParams;

impl ObjectAQLParams {
    pub const QL_QUERY: crate::Field<Self, crate::Text> = crate::Field::new("qlQuery");
}
