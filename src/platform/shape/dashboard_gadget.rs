#[derive(Clone, Copy, Debug)]
pub struct DashboardGadget;

impl DashboardGadget {
    pub const COLOR: crate::Field<Self, crate::Text> = crate::Field::new("color");
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const MODULE_KEY: crate::Field<Self, crate::Text> = crate::Field::new("moduleKey");

    pub const POSITION: crate::Field<Self, crate::Object<super::DashboardGadgetPosition>> =
        crate::Field::new("position");

    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");
    pub const URI: crate::Field<Self, crate::Text> = crate::Field::new("uri");
}
