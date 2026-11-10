#[derive(Clone, Copy, Debug)]
pub struct SuggestedMappingsForPrioritiesRequestBean;

impl SuggestedMappingsForPrioritiesRequestBean {
    pub const ADD: crate::Field<Self, crate::List<crate::Integer>> = crate::Field::new("add");
    pub const REMOVE: crate::Field<Self, crate::List<crate::Integer>> = crate::Field::new("remove");
}
