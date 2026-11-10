#[derive(Clone, Copy, Debug)]
pub struct ForgePanelProjectPinAsyncResponse;

impl ForgePanelProjectPinAsyncResponse {
    pub const TASK_ID: crate::Field<Self, crate::Text> = crate::Field::new("taskId");
}
