#[derive(Clone, Copy, Debug)]
pub struct IssueFieldOptionScopeBean;

impl IssueFieldOptionScopeBean {
    pub const GLOBAL: crate::Field<Self, crate::Object<super::GlobalScopeBean>> =
        crate::Field::new("global");

    pub const PROJECTS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("projects");

    pub const PROJECTS2: crate::Field<Self, crate::List<crate::Object<super::ProjectScopeBean>>> =
        crate::Field::new("projects2");
}
