#[derive(Clone, Copy, Debug)]
pub struct ForgePanelProjectPinRequest;

impl ForgePanelProjectPinRequest {
    pub const MODULE_ID: crate::Field<Self, crate::Text> = crate::Field::new("moduleId");

    pub const PROJECT_LIST: crate::Field<
        Self,
        crate::List<crate::Object<super::ProjectPinAction>>,
    > = crate::Field::new("projectList");
}
