#[derive(Clone, Copy, Debug)]
pub struct ProjectScopeBean;

impl ProjectScopeBean {
    pub const ATTRIBUTES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("attributes");

    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
}
