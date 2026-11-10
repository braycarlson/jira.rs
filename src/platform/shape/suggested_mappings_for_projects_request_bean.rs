#[derive(Clone, Copy, Debug)]
pub struct SuggestedMappingsForProjectsRequestBean;

impl SuggestedMappingsForProjectsRequestBean {
    pub const ADD: crate::Field<Self, crate::List<crate::Integer>> = crate::Field::new("add");
}
