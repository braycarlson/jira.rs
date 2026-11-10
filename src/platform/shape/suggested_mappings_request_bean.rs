#[derive(Clone, Copy, Debug)]
pub struct SuggestedMappingsRequestBean;

impl SuggestedMappingsRequestBean {
    pub const PRIORITIES: crate::Field<
        Self,
        crate::Object<super::SuggestedMappingsForPrioritiesRequestBean>,
    > = crate::Field::new("priorities");

    pub const PROJECTS: crate::Field<
        Self,
        crate::Object<super::SuggestedMappingsForProjectsRequestBean>,
    > = crate::Field::new("projects");

    pub const RESULTS_MAX: crate::Field<Self, crate::Integer> = crate::Field::new("maxResults");
    pub const SCHEME_ID: crate::Field<Self, crate::Integer> = crate::Field::new("schemeId");
    pub const START_AT: crate::Field<Self, crate::Integer> = crate::Field::new("startAt");
}
