#[derive(Clone, Copy, Debug)]
pub struct PartialSuccessBean;

impl PartialSuccessBean {
    pub const ENTRIES: crate::Field<
        Self,
        crate::List<crate::Object<super::PartialSuccessBeanEntries>>,
    > = crate::Field::new("entries");
}
