#[derive(Clone, Copy, Debug)]
pub struct IssueFieldOptionConfiguration;

impl IssueFieldOptionConfiguration {
    pub const ATTRIBUTES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("attributes");

    pub const SCOPE: crate::Field<Self, crate::Object<super::IssueFieldOptionScopeBean>> =
        crate::Field::new("scope");
}
