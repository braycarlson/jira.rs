#[derive(Clone, Copy, Debug)]
pub struct AvailableDashboardGadget;

impl AvailableDashboardGadget {
    pub const MODULE_KEY: crate::Field<Self, crate::Text> = crate::Field::new("moduleKey");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");
    pub const URI: crate::Field<Self, crate::Text> = crate::Field::new("uri");
}
