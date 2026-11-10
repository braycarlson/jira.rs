#[derive(Clone, Copy, Debug)]
pub struct IssueBeanEditmeta;

impl IssueBeanEditmeta {
    pub const FIELDS: crate::Field<
        Self,
        crate::Map<crate::Object<super::IssueBeanEditmetaFields>>,
    > = crate::Field::new("fields");
}
