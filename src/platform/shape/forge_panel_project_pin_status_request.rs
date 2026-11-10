#[derive(Clone, Copy, Debug)]
pub struct ForgePanelProjectPinStatusRequest;

impl ForgePanelProjectPinStatusRequest {
    pub const MODULE_ID: crate::Field<Self, crate::Text> = crate::Field::new("moduleId");

    pub const PROJECT_LIST: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("projectList");
}
