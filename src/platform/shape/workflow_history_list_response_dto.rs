#[derive(Clone, Copy, Debug)]
pub struct WorkflowHistoryListResponseDTO;

impl WorkflowHistoryListResponseDTO {
    pub const ENTRIES: crate::Field<
        Self,
        crate::List<crate::Object<super::WorkflowHistoryItemDTO>>,
    > = crate::Field::new("entries");
}
