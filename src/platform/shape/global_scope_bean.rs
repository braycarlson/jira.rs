#[derive(Clone, Copy, Debug)]
pub struct GlobalScopeBean;

impl GlobalScopeBean {
    pub const ATTRIBUTES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("attributes");
}
