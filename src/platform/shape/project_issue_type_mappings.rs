#[derive(Clone, Copy, Debug)]
pub struct ProjectIssueTypeMappings;

impl ProjectIssueTypeMappings {
    pub const MAPPINGS: crate::Field<
        Self,
        crate::List<crate::Object<super::ProjectIssueTypeMapping>>,
    > = crate::Field::new("mappings");
}
