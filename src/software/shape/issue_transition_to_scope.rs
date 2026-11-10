#[derive(Clone, Copy, Debug)]
pub struct IssueTransitionToScope;

impl IssueTransitionToScope {
    pub const PROJECT: crate::Field<Self, crate::Object<super::IssueTransitionToScopeProject>> =
        crate::Field::new("project");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
