#[derive(Clone, Copy, Debug)]
pub struct WorkflowHistoryReadResponseDTO;

impl WorkflowHistoryReadResponseDTO {
    pub const STATUSES: crate::Field<
        Self,
        crate::List<crate::Object<super::WorkflowDocumentStatusDTO>>,
    > = crate::Field::new("statuses");

    pub const WORKFLOWS: crate::Field<
        Self,
        crate::List<crate::Object<super::WorkflowDocumentDTO>>,
    > = crate::Field::new("workflows");
}
