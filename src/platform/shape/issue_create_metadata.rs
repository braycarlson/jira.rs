#[derive(Clone, Copy, Debug)]
pub struct IssueCreateMetadata;

impl IssueCreateMetadata {
    pub const EXPAND: crate::Field<Self, crate::Text> = crate::Field::new("expand");

    pub const PROJECTS: crate::Field<
        Self,
        crate::List<crate::Object<super::ProjectIssueCreateMetadata>>,
    > = crate::Field::new("projects");
}
