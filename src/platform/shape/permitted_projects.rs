#[derive(Clone, Copy, Debug)]
pub struct PermittedProjects;

impl PermittedProjects {
    pub const PROJECTS: crate::Field<
        Self,
        crate::List<crate::Object<super::ProjectIdentifierBean>>,
    > = crate::Field::new("projects");
}
