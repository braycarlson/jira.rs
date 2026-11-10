#[derive(Clone, Copy, Debug)]
pub struct ForgePanelProjectPinStatusResponse;

impl ForgePanelProjectPinStatusResponse {
    pub const MODULE_ID: crate::Field<Self, crate::Text> = crate::Field::new("moduleId");

    pub const STATUSES: crate::Field<
        Self,
        crate::List<crate::Object<super::ForgePanelProjectPinStatus>>,
    > = crate::Field::new("statuses");
}
