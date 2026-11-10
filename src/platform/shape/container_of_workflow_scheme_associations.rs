#[derive(Clone, Copy, Debug)]
pub struct ContainerOfWorkflowSchemeAssociations;

impl ContainerOfWorkflowSchemeAssociations {
    pub const VALUES: crate::Field<
        Self,
        crate::List<crate::Object<super::WorkflowSchemeAssociations>>,
    > = crate::Field::new("values");
}
